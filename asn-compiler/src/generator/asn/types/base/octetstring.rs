//! Generator code for Base Type Asn1ResolvedOctetString

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::error::Error;

use crate::generator::Generator;
use crate::resolver::asn::structs::types::base::Asn1ResolvedOctetString;

impl Asn1ResolvedOctetString {
    pub(crate) fn generate(
        &self,
        name: &str,
        generator: &mut Generator,
    ) -> Result<TokenStream, Error> {
        let struct_name = generator.to_type_ident(name);

        let mut ty_attributes = quote! { type = "OCTET-STRING" };

        if self.size.is_some() {
            let sz_attributes = self.size.as_ref().unwrap().get_ty_size_constraints_attrs();
            ty_attributes.extend(sz_attributes);
        }

        let struct_tokens = quote! {
            #[derive(Debug, AperCodec)]
            #[asn(#ty_attributes)]
            pub struct #struct_name(pub Vec<u8>);
        };

        Ok(struct_tokens)
    }

    pub(crate) fn generate_ident_and_aux_type(
        &self,
        generator: &mut Generator,
    ) -> Result<Ident, Error> {
        let unique_name = generator.get_unique_name("OCTET STRING");

        let item = self.generate(&unique_name, generator)?;
        generator.aux_items.push(item);

        Ok(generator.to_type_ident(&unique_name))
    }
}
