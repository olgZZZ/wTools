#[ allow( unused_imports ) ]
use super::*;

// trace_macros!( true );
TheModule::types!
{

  ///
  /// Attribute which is inner.
  ///

  #[ derive( Debug, Clone ) ]
  #[ derive( PartialEq ) ]
  pair Pair : < T1 : core::cmp::PartialEq + core::clone::Clone >;

}
// trace_macros!( false );

include!( "./homo_pair_parameter_main_test_only.rs" );
