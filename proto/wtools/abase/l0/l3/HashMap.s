( function _l3_HashMap_s_()
{

'use strict';

const _global = _global_;
const _ = _global_.wTools;
const Self = _global_.wTools.hashMap = _global_.wTools.hashMap || Object.create( null );

// --
// implementation
// --

function is( src )
{
  if( !src )
  return false;
  return src instanceof HashMap || src instanceof HashMapWeak;
}

//

function like( src )
{
  return _.hashMap.is( src );
}

//

function isEmpty()
{
  return !src.size;
}

//

function isPopulated()
{
  return !!src.size;
}

//

function identicalShallow( src1, src2 )
{
  _.assert( arguments.length === 2, 'Expects exactly two arguments' );
  // _.assert( _.hashMap.like( src1 ) );
  // _.assert( _.hashMap.like( src2 ) );

  if( !_.hashMap.like( src1 ) )
  return false;
  if( !_.hashMap.like( src2 ) )
  return false;

  return _.hashMap._identicalShallow( src1, src2 );
}

//

function _identicalShallow( src1, src2 )
{

  let testVal;

  if( src1.size !== src2.size )
  return false;

  for( let [ key, val ] of src1 )
  {
    testVal = src2.get( key );
    /*
      in cases of an undefined value, make sure the key
      exists on the object so there are no false positives
    */
    if( testVal !== val || ( testVal === undefined && !src2.has( key ) ) )
    return false;
  }
  return true;
}

//

function exportStringShallowDiagnostic( src )
{
  _.assert( arguments.length === 1, 'Expects exactly one argument' );
  _.assert( _.hashMap.is( src ) );

  return `{- ${_.entity.strType( src )} with ${_.entity.lengthOf( src )} elements -}`;
}

// --
// extension
// --

let ToolsExtension =
{
  hashMapIs : is,
  hashMapLike : like,
  hashMapIsEmpty : isEmpty,
  hashMapIsPopulated : isPopulated,
}

//

let Extension =
{
  is,
  like,
  isEmpty,
  isPopulated,
  identicalShallow,
  _identicalShallow,
  equivalentShallow : identicalShallow,

  // export string

  exportString : exportStringShallowDiagnostic,
  exportStringShallow : exportStringShallowDiagnostic,
  exportStringShallowDiagnostic,
  exportStringShallowCode : exportStringShallowDiagnostic,
  exportStringDiagnostic : exportStringShallowDiagnostic,
  exportStringCode : exportStringShallowDiagnostic,

}

Object.assign( _, ToolsExtension );
Object.assign( _.hashMap, Extension );

})();
