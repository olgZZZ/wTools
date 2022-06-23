use std::env::current_dir;
use std::path::PathBuf;
use wtools::error::BasicError;
use wca::instruction::Instruction;
use ::wpublisher::manifest::Manifest;

///
/// Perform smoke testing.
///

pub fn smoke( instruction : &Instruction ) -> Result< (), BasicError >
{
  let mut current_path = current_dir().unwrap();

  let subject_path = PathBuf::from( &instruction.subject );
  let module_path = if subject_path.is_relative()
  {
    current_path.push( &instruction.subject );
    current_path
  }
  else
  {
    subject_path
  };

  let mut manifest_path = module_path.clone();
  manifest_path.push( "Cargo.toml" );

  if !manifest_path.exists()
  {
    let msg = format!( "Current directory \"{:?}\" has no file \"Cargo.toml\"", module_path );
    return Err( BasicError::new( msg ) );
  }

  let mut manifest = Manifest::new();
  manifest.manifest_path_from_str( &manifest_path ).unwrap();
  manifest.load().unwrap();
  let data = manifest.manifest_data.as_deref().unwrap();

  /* */

  let module_name = &data[ "package" ][ "name" ].clone();
  let module_name = module_name.as_str().unwrap();

  let code_path = match instruction.properties_map.get( "code_path" )
  {
    Some( x ) => PathBuf::from( x.clone().primitive().unwrap() ),
    None => PathBuf::default(),
  };

  let mut data = None;
  if code_path.exists()
  {
    data = Some( std::fs::read_to_string( code_path ).unwrap() );
  }

  let version = match instruction.properties_map.get( "version" )
  {
    Some( x ) => x.clone().primitive().unwrap(),
    None => "*".to_string(),
  };

  let smoke = match instruction.properties_map.get( "smoke" )
  {
    Some( x ) => x.clone().primitive().unwrap(),
    None =>
    {
      if let Ok( x ) = std::env::var( "WITH_SMOKE" )
      {
        x
      }
      else
      {
        "local".to_string()
      }
    },
  };

  /* */

  if smoke != "false" && smoke != "0"
  {

    if smoke == "local" || smoke != "published"
    {
      let mut t = SmokeModuleTest::new( module_name );
      t.test_postfix( "_test_local" );
      if data.is_some()
      {
        t.code( data.as_ref().unwrap() );
      }
      t.version( version.as_str() );
      t.local_path_clause( module_path.to_str().unwrap() );

      t.clean( true ).unwrap();
      t.form().unwrap();
      t.perform().unwrap();
      t.clean( false ).unwrap();
    }

    if smoke == "published" || smoke != "local"
    {
      let mut t = SmokeModuleTest::new( module_name );
      t.test_postfix( "_test_published" );
      if data.is_some()
      {
        t.code( data.as_ref().unwrap() );
      }
      t.version( version.as_str() );

      t.clean( true ).unwrap();
      t.form().unwrap();
      t.perform().unwrap();
      t.clean( false ).unwrap();
    }
  }

  Ok( () )
}

//

#[ derive( Debug ) ]
struct SmokeModuleTest< 'a >
{
  pub dependency_name : &'a str,
  pub version : &'a str,
  pub local_path_clause : &'a str,
  pub code : String,
  pub test_path : std::path::PathBuf,
  pub test_postfix : &'a str,
}

impl< 'a > SmokeModuleTest< 'a >
{
  fn new( dependency_name : &'a str ) -> SmokeModuleTest< 'a >
  {
    let test_postfix = "_smoke_test";
    let smoke_test_path = format!( "{}{}", dependency_name, test_postfix );
    let mut test_path = std::env::temp_dir();
    test_path.push( smoke_test_path );

    SmokeModuleTest
    {
      dependency_name,
      version : "*",
      local_path_clause : "",
      code : "".to_string(),
      test_path,
      test_postfix,
    }
  }

  fn version( &mut self, version : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.version = version.as_ref();
    self
  }

  fn local_path_clause( &mut self, local_path_clause : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.local_path_clause = local_path_clause;
    self
  }

  fn test_postfix( &mut self, test_postfix : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.test_postfix = test_postfix;
    let smoke_test_path = format!( "{}{}", self.dependency_name, test_postfix );
    self.test_path.pop();
    self.test_path.push( smoke_test_path );
    self
  }

  fn code( &mut self, code : impl AsRef< str > + 'a ) -> &mut SmokeModuleTest< 'a >
  {
    self.code = code.as_ref().into();
    self
  }

  fn form( &mut self ) -> Result< (), &'static str >
  {
    std::fs::create_dir( &self.test_path ).unwrap();

    let mut test_path = self.test_path.clone();

    /* create binary test module */
    let test_name = format!( "{}{}", self.dependency_name, self.test_postfix );
    let output = std::process::Command::new( "cargo" )
    .current_dir( &test_path )
    .args([ "new", "--bin", &test_name ])
    .output()
    .expect( "Failed to execute command" );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );

    test_path.push( test_name );

    /* setup config */
    #[ cfg( target_os = "windows" ) ]
    let local_path_clause = if self.local_path_clause == "" { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause.escape_default() ) };
    #[ cfg( not( target_os = "windows" ) ) ]
    let local_path_clause = if self.local_path_clause == "" { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause ) };
    let dependencies_section = format!( "{} = {{ version = \"{}\" {} }}", self.dependency_name, self.version, &local_path_clause );
    let config_data = format!
    (
      "[package]
      edition = \"2021\"
      name = \"{}_smoke_test\"
      version = \"0.0.1\"

      [dependencies]
      {}",
      &self.dependency_name,
      &dependencies_section
    );
    let mut config_path = test_path.clone();
    config_path.push( "Cargo.toml" );
    println!( "\n{}\n", config_data );
    std::fs::write( config_path, config_data ).unwrap();

    /* write code */
    test_path.push( "src" );
    test_path.push( "main.rs" );
    if self.code == ""
    {
      self.code = format!( "use ::{}::*;", self.dependency_name );
    }
    let code = format!
    (
      "#[ allow( unused_imports ) ]
      fn main()
      {{
        {}
      }}",
      self.code,
    );
    println!( "\n{}\n", code );
    std::fs::write( &test_path, code ).unwrap();

    Ok( () )
  }

  fn perform( &self ) -> Result<(), BasicError>
  {
    let mut test_path = self.test_path.clone();
    let test_name = format!( "{}{}", self.dependency_name, self.test_postfix );
    test_path.push( test_name );

    let output = std::process::Command::new( "cargo" )
    .current_dir( test_path )
    .args([ "run", "--release" ])
    .output()
    .unwrap();
    println!( "status : {}", output.status );
    println!( "{}", std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" ) );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );
    if !output.status.success()
    {
      return Err( BasicError::new( "Smoke test failed" ) );
    }

    Ok( () )
  }

  fn clean( &self, force : bool ) -> Result<(), &'static str>
  {
    let result = std::fs::remove_dir_all( &self.test_path );
    if force
    {
      result.unwrap_or_default();
    }
    else
    {
      let msg = format!( "Cannot remove temporary directory {}. Please, remove it manually", &self.test_path.display() );
      result.expect( &msg );
    }
    Ok( () )
  }
}

