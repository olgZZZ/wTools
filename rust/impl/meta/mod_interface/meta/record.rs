/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use derive_tools::IsVariant;
  use proc_macro_tools::exposed::*;

  /// Custom keywords.
  pub mod kw
  {
    super::syn::custom_keyword!( layer );
  }

  /// Kind of element.
  #[ derive( IsVariant, Debug, PartialEq, Eq, Clone, Copy ) ]
  pub enum ElementType
  {
    MicroModule( syn::token::Mod ),
    Layer( kw::layer ),
  }

  //

  impl syn::parse::Parse for ElementType
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {
      let element_type;
      let lookahead = input.lookahead1();

      if lookahead.peek( syn::token::Mod )
      {
        element_type = ElementType::MicroModule( input.parse()? );
      }
      else
      {
        element_type = ElementType::Layer( input.parse()? );
      }

      Ok( element_type )
    }

  }

  //

  impl quote::ToTokens for ElementType
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use ElementType::*;
      match self
      {
        MicroModule( e ) => e.to_tokens( tokens ),
        Layer( e ) => e.to_tokens( tokens ),
      }
    }
  }

  ///
  /// Record.
  ///

  #[ derive( Debug, PartialEq, Eq, Clone ) ]
  pub struct Record
  {
    pub attrs : Vec< syn::Attribute >,
    pub vis : Visibility,
    pub element_type : ElementType,
    pub elements : syn::punctuated::Punctuated< syn::Ident, syn::token::Comma >,
    pub semi : Option< syn::token::Semi >,
  }

  //

//   pub fn attrs_parse_inner_single( input : ParseStream< '_ > ) -> Result< syn::Attribute >
//   {
//     let input2;
//     Ok( syn::Attribute
//     {
//       pound_token : input.parse()?,
//       style : syn::AttrStyle::Inner( input.parse()? ),
//       bracket_token : bracketed!( input2 in input ),
//       path : input2.call( syn::Path::parse_mod_style )?,
//       tokens : input2.parse()?,
//     })
//   }
//
//   //
//
//   pub fn attrs_parse_inner_as_much_as_possible
//   (
//     input : ParseStream< '_ >,
//     attrs : &mut Vec< syn::Attribute >,
//   )
//   -> Result< () >
//   {
//     while input.peek( Token![ # ] ) && input.peek2( Token![ ! ] )
//     {
//       attrs.push( input.call( attrs_parse_inner_single )? );
//       // attrs.push( input.call( parsing::single_parse_inner )? );
//     }
//     Ok( () )
//   }

  //

  impl syn::parse::Parse for Record
  {

    fn parse( input : ParseStream< '_ > ) -> Result< Self >
    {

      let attrs = input.call( syn::Attribute::parse_outer )?;
      let vis : Visibility = input.parse()?;
      let element_type : ElementType = input.parse()?;
      let mut elements;

      let lookahead = input.lookahead1();
      if lookahead.peek( syn::token::Brace )
      {
        let input2;
        let _brace_token = syn::braced!( input2 in input );
        elements = syn::punctuated::Punctuated::< _, _ >::parse_terminated( &input2 )?;
      }
      else
      {
        let ident : syn::Ident = input.parse()?;
        elements = syn::punctuated::Punctuated::new();
        elements.push( ident );
      }

      let lookahead = input.lookahead1();
      if !lookahead.peek( Token![ ; ] )
      {
        return Err( lookahead.error() );
      }

      let semi = Some( input.parse()? );
      return Ok( Record
      {
        attrs,
        vis,
        element_type,
        elements,
        semi,
      })

    }

  }

//   impl syn::parse::Parse for Record
//   {
//     fn parse( input : ParseStream< '_ > ) -> Result< Self >
//     {
//
//       let attrs = input.call( syn::Attribute::parse_outer )?;
//       let vis : Visibility = input.parse()?;
//       let mod_token : Option< Token![ mod ] > = input.parse()?;
//
//       let lookahead = input.lookahead1();
//       if lookahead.peek( syn::token::Brace )
//       {
//         let input2;
//         let _brace_token = syn::braced!( input2 in input );
//         let elements = syn::punctuated::Punctuated::< _, _ >::parse_terminated( &input2 )?;
//
//         let semi = Some( input.parse()? );
//
//         return Ok( Record
//         {
//           attrs,
//           vis,
//           mod_token,
//           elements,
//           semi,
//         })
//       }
//
//       let ident : syn::Ident = input.parse()?;
//       let lookahead = input.lookahead1();
//       if lookahead.peek( Token![ ; ] )
//       {
//         let mut elements = syn::punctuated::Punctuated::new();
//         elements.push( ident );
//
//         Ok( Record
//         {
//           attrs,
//           vis,
//           mod_token,
//           elements,
//           semi : Some( input.parse()? ),
//         })
//       }
// //       else if lookahead.peek( syn::token::Brace )
// //       {
// //         let input2;
// //         let brace_token = syn::braced!( input2 in input );
// //         attrs_parse_inner_as_much_as_possible( &input2, &mut attrs )?;
// //
// //         let mut items = Vec::new();
// //         while !input2.is_empty()
// //         {
// //           items.push( input2.parse()? );
// //         }
// //
// //         Ok( Record
// //         {
// //           attrs,
// //           vis,
// //           mod_token,
// //           ident,
// //           content : Some( ( brace_token, items ) ),
// //           semi : None,
// //         })
// //       }
//       else
//       {
//         Err( lookahead.error() )
//       }
//
//     }
//
//   }

  //

  impl quote::ToTokens for Record
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use proc_macro_tools::quote::TokenStreamExt;
      tokens.append_all( &self.attrs );
      self.vis.to_tokens( tokens );
      self.element_type.to_tokens( tokens );
      self.elements.to_tokens( tokens );
      self.semi.to_tokens( tokens );
    }
  }

  ///
  /// Module-specific item.
  ///

  #[ derive( Debug ) ]
  pub struct Records
  (
    pub Vec< Record >,
  );

  //

  impl syn::parse::Parse for Records
  {
    fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
    {
      let mut items = vec![];
      while !input.is_empty()
      {
        let item : Record = input.parse()?;
        items.push( item );
      }
      Ok( Self( items ) )
    }
  }

  //

  impl quote::ToTokens for Records
  {
    fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
    {
      use proc_macro_tools::quote::TokenStreamExt;
      tokens.append_all( &self.0 )
    }
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::private::
  {
    Record,
    Records,
    ElementType,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
