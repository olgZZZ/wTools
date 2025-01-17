#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wtest_basic/latest/wtest_basic/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

// doc_file_test!( "rust/test/test/asset/Test.md" );

/// Dependencies.
pub mod dependency
{
  #[ doc( inline ) ]
  pub use ::paste;
  #[ doc( inline ) ]
  pub use ::trybuild;
  #[ doc( inline ) ]
  pub use ::anyhow;
  #[ doc( inline ) ]
  pub use ::rustversion;
  #[ doc( inline ) ]
  pub use ::meta_tools;
  #[ doc( inline ) ]
  pub use ::mem_tools;
  #[ doc( inline ) ]
  pub use ::typing_tools;
  #[ doc( inline ) ]
  pub use ::num_traits;
  #[ doc( inline ) ]
  pub use ::diagnostics_tools;
}

use ::meta_tools::mod_interface;

mod_interface!
{
  /// Basics.
  layer basic;

  // use super::exposed::meta;
  use super::exposed::mem;
  use super::exposed::typing;
  use super::exposed::dt;
  use super::exposed::diagnostics;

  protected use super::dependency;
  protected use super::dependency::*;

  prelude use ::meta_tools as meta;
  prelude use ::mem_tools as mem;
  prelude use ::typing_tools as typing;
  prelude use ::data_type as dt;
  prelude use ::diagnostics_tools as diagnostics;

  prelude use ::meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };
  prelude use ::typing_tools::{ implements };

}

// qqq : for Dima : add negative test that wtest_basic::exposed::exposed does not exist /* aaa : Dmytro : added trybuild test with compile time error */
