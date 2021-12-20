
pub use std::error::Error as ErrorTrait;

/// baic implementation of generic Error

#[ derive( core::fmt::Debug, core::clone::Clone, core::cmp::PartialEq ) ]
pub struct Error
{
  msg : String,
}

impl Error
{
  pub fn new< Msg : Into< String > >( msg : Msg ) -> Error
  {
    Error { msg : msg.into() }
  }
  pub fn msg( &self ) -> &String
  {
    &self.msg
  }
}

impl core::fmt::Display for Error
{
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result
  {
    write!( f, "{}", self.msg )
  }
}

impl std::error::Error for Error
{
  fn description( &self ) -> &str
  {
    &self.msg
  }
}