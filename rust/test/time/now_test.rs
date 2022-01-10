
use wtest_basic::*;

//

fn _basic()
{
  use time_tools::*;

  // test.case( "time::now" );
  let got = time::now();
  assert!( got > 0 );

  // test.case( "time::ms::now" );
  let got1 = time::now();
  let got2 = time::ms::now();
  assert_eq!( got1, got2 );

  // test.case( "time::ns::now" );
  let got1 = time::now();
  let got2 = time::ns::now();
  assert_eq!( got1, got2 / 1000000 );

  // test.case( "time::s::now" );
  let got1 = time::now();
  let got2 = time::s::now();
  assert_eq!( got1 / 1000, got2 );

}

//

test_suite!
{
  basic,
}
