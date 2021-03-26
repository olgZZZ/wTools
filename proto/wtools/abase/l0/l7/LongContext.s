( function _LongContext_s_()
{

'use strict';

const _global = _global_;
const _ = _global.wTools;
const Self = _global.wTools;

_.assert( !_.Array );
_.assert( !_.defaultLong );
_.assert( !_.withDefaultLong );

// --
//
// --

function applyTo( dst, def )
{

  _.assert( arguments.length === 2, 'Expects exactly two arguments' );
  _.assert( !!_.LongDescriptors[ def ] );

  dst.withDefaultLong = Object.create( null ); /* xxx */

  for( let d in _.LongDescriptors )
  {
    let context = dst.withDefaultLong[ d ] = Object.create( dst );
    context.longDescriptor = _.LongDescriptors[ d ];
  }

  dst.longDescriptor = _.LongDescriptors[ def ];
  _.assert( _.mapIs( dst.longDescriptor ) );

}

// --
// delcare
// --

let longDescriptorProducer =
{
  applyTo,
}

_.longDescriptorProducer = _.longDescriptorProducer || Object.create( null );
_.mapExtend( _.longDescriptorProducer, longDescriptorProducer );

let LongContext = Object.create( null );
let ToolsExtension =
{
  LongContext,
}

_.mapExtend( _, ToolsExtension );

// --
//
// --

/**
 * @summary Array namespace
 * @namespace wTools.defaultLong
 * @extends Tools
 * @namespace Tools
 */

_.assert( !_.Array );
_.assert( !_.defaultLong );
_.assert( !_.withDefaultLong );

_.longDescriptorProducer.applyTo( _, 'Array' );

_.assert( !_.Array );
_.assert( _.object.is( _.withDefaultLong ) );
_.assert( !_.defaultLong );

_.assert( _.longDescriptorProducer );
_.assert( _.LongDescriptors );
_.assert( _.longDescriptor );

})();
