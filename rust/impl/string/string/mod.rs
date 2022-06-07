
/// Add indentation to each line.
#[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
pub mod indentation;
/// Isolate parts of string.
#[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
pub mod isolate;
/// Spit string with a delimeter.
#[ cfg( all( feature = "split", feature = "use_std" ) ) ]
pub mod split;
/// Parse string.
#[ cfg( all( feature = "parse", feature = "use_std" ) ) ]
pub mod parse;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::orphan::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::orphan::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::orphan::*;
  #[ cfg( all( feature = "parse", feature = "use_std" ) ) ]
  pub use super::parse::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::exposed::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::exposed::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::exposed::*;
  #[ cfg( all( feature = "parse", feature = "use_std" ) ) ]
  pub use super::parse::exposed::*;
}

pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::prelude::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::prelude::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::prelude::*;
  #[ cfg( all( feature = "parse", feature = "use_std" ) ) ]
  pub use super::parse::prelude::*;
}
