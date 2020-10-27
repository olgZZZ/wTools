( function _gTime_s_()
{

'use strict';

let _global = _global_;
let _ = _global_.wTools;
let Self = _global_.wTools.time = _global_.wTools.time || Object.create( null );

// --
// implementation
// --

function rarely_functor( perTime, routine )
{
  let lastTime = _.time.now() - perTime;

  _.assert( arguments.length === 2 );
  _.assert( _.numberIs( perTime ) );
  _.assert( _.routineIs( routine ) );

  return function fewer()
  {
    let now = _.time.now();
    let elapsed = now - lastTime;
    if( elapsed < perTime )
    return;
    lastTime = now;
    return routine.apply( this, arguments );
  }

}

// //
//
// function periodic( delay, onReady )
// {
//   _.assert( _.routineIs( _.Consequence ) );
//
//   let con = new _.Consequence();
//   let id;
//
//   _.assert( arguments.length === 2, 'Expects exactly two arguments' );
//   _.assert( _.numberIs( delay ) );
//
//   let _onReady = null;
//
//   if( _.routineIs( onReady ) )
//   _onReady = function()
//   {
//     let result = onReady.call();
//     if( result === false )
//     clearInterval( id );
//     _.Consequence.take( con, undefined );
//     con.finally( handlePeriodicCon );
//   }
//   else if( onReady instanceof wConsquence )
//   _onReady = function()
//   {
//     let result = onReady.ping();
//     if( result === false )
//     clearInterval( id );
//     _.Consequence.take( con, undefined );
//     con.finally( handlePeriodicCon );
//   }
//   else if( onReady === undefined )
//   _onReady = function()
//   {
//     _.Consequence.take( con, undefined );
//     con.finally( handlePeriodicCon );
//   }
//   else throw _.err( 'unexpected type of onReady' );
//
//   id = setInterval( _onReady, delay );
//
//   return con;
//
//   function handlePeriodicCon( err )
//   {
//     if( arg === _.dont )
//     clearInterval( id );
//     // if( err )
//     // clearInterval( id );
//     /* zzz */
//   }
//
// }

//

function once( delay, onBegin, onEnd )
{
  let con = _.Consequence ? new _.Consequence({ /* sourcePath : 2 */ }) : undefined;
  let taken = false;
  let options;
  let optionsDefault =
  {
    delay : null,
    onBegin : null,
    onEnd : null,
  }

  if( _.objectIs( delay ) )
  {
    options = delay;
    _.assert( arguments.length === 1, 'Expects single argument' );
    _.assertMapHasOnly( options, optionsDefault );
    delay = options.delay;
    onBegin = options.onBegin;
    onEnd = options.onEnd;
  }
  else
  {
    _.assert( 2 <= arguments.length && arguments.length <= 3 );
  }

  // _.assert( 0, 'not tested' );
  _.assert( delay >= 0 );
  _.assert( _.primitiveIs( onBegin ) || _.routineIs( onBegin ) || _.objectIs( onBegin ) );
  _.assert( _.primitiveIs( onEnd ) || _.routineIs( onEnd ) || _.objectIs( onEnd ) );

  return function once()
  {

    if( taken )
    {
      /*console.log( 'once :', 'was taken' );*/
      return;
    }
    taken = true;

    if( onBegin )
    {
      if( _.routineIs( onBegin ) ) onBegin.apply( this, arguments );
      else if( _.objectIs( onBegin ) ) onBegin.take( arguments );
      if( con )
      con.take( null );
    }

    _.time.out( delay, function()
    {

      if( onEnd )
      {
        if( _.routineIs( onEnd ) ) onEnd.apply( this, arguments );
        else if( _.objectIs( onEnd ) ) onEnd.take( arguments );
        if( con )
        con.take( null );
      }
      taken = false;

    });

    return con;
  }

}

//

function debounce( o ) /* Dmytro : routine returns routine. Is it valid result? */
{
  _.assert( arguments.length <= 3 );

  if( arguments.length > 1 )
  {
    o =
    {
      routine : arguments[ 0 ],
      delay : arguments[ 1 ],
      immediate : arguments[ 2 ],
    }
  }

  _.routineOptions( debounce, o  );

  _.assert( _.routineIs( o.routine ) );
  _.assert( _.numberIs( o.delay ) );

  let timer, lastCallTime, routine, result;

  return debounced;

  /* */

  function debounced()
  {
    lastCallTime = _.time.now();
    routine = _.routineJoin( this, o.routine, arguments );
    let execNow = o.immediate && !timer
    if( !timer )
    timer = setTimeout( onDelay, o.delay );
    if( execNow )
    result = routine();
    return result;
  };

  function onDelay()
  {
    var elapsed = _.time.now() - lastCallTime;

    if( elapsed > o.delay )
    {
      timer = null;
      if( !o.immediate )
      result = routine();
    }
    else
    {
      timer = setTimeout( onDelay, o.delay - elapsed );
    }
  };
}

debounce.defaults =
{
  routine : null,
  delay : 100,
  immediate : false
}


// --
// fields
// --

let Fields =
{
}

// --
// routines
// --

let Routines =
{

  // ready, /* Dmytro : moved to gProcess.s */
  // readyJoin, /* Dmytro : moved to module Consequence */

  // out,
  // outError,
  //
  // _errTimeOut,

  rarely_functor, /* check */
  // periodic, /* dubious */
  once, /* qqq : cover by light test */

  debounce

}

//

_.mapSupplement( Self, Fields );
_.mapSupplement( Self, Routines );

// --
// export
// --

if( typeof module !== 'undefined' )
module[ 'exports' ] = _;

})();
