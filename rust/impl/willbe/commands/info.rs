/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };
  use wca::
  {
    Args,
    NoSubject,
    NoProperties,
  };

  ///
  /// Prints information about package
  ///

  pub fn info( _ : Args< NoSubject, NoProperties >, ctx : wca::Context ) -> Result< (), BasicError >
  {
    let current_path = if let Some( path ) = ctx.get_ref::< std::path::PathBuf >()
    {
      path.to_owned()
    }
    else
    {
      env::current_dir().unwrap().to_owned()
    };

    println!( "[LOG] Called info command" );

    let package = Package::try_from( current_path )
    .map_err( | _ | err!( "Package not found at current directory" ) )?;

    let info = PackageMetadata::try_from( package )
    .map_err( | _ | err!( "Can not parse package metadata" ) )?;
    let info = info.all().to_owned();

    println!
    (
      r#"
Name: "{}"
Version: "{}"
Description: "{}"
Documentation: "{}"
License: "{}"
Readme: "{}"
Dependencies: {:?}
Location: "{}"
      "#,
      info.name,
      info.version,
      info.description.unwrap_or_else( || "Not found".to_string() ),
      info.documentation.unwrap_or_else( || "Not found".to_string() ),
      info.license.unwrap_or_else( || "Not found".to_string() ),
      info.readme.map( String::from ).unwrap_or_else( || "Not found".to_string() ),
      info.dependencies.iter().map( | d | &d.name ).collect::< Vec< _ > >(),
      info.manifest_path.parent().unwrap()
    );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use info;
}
