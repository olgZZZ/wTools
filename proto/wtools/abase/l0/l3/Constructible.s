( function _l3_Constructible_s_()
{

'use strict';

const _global = _global_;
const _ = _global_.wTools;
const Self = _global_.wTools.constructible = _global_.wTools.constructible || Object.create( null );

// --
// typing
// --

function is( src ) /* xxx qqq : optimize */
{
  if( _.primitive.is( src ) )
  return false;

  let proto = Object.getPrototypeOf( src );
  if( proto === null )
  return false;

  if( !Reflect.has( proto, 'constructor' ) )
  return false;
  if( proto.constructor === Object )
  return false;

  if( _.aux.is( src ) ) /* xxx : remove? */
  return false;
  if( _.vector.is( src ) )
  return false;
  if( _.set.is( src ) )
  return false;
  if( _.hashMap.is( src ) )
  return false;

  return true;
}

//

function like( src )
{
  return _.constructibleIs( src );
}

// --
// extension
// --

let ToolsExtension =
{

  // typing

  constructibleIs : is, /* qqq : cover and move */
  constructibleLike : like, /* qqq : cover and move */

}

//

let Extension =
{

  // typing

  is, /* qqq : cover and move */
  like, /* qqq : cover and move */

}

//

Object.assign( _, ToolsExtension );
Object.assign( Self, Extension );

})();
