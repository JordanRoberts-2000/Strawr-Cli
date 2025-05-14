use crate::template::{
    models::{Template, Variant},
    types::{ParsedTemplateInput, ValidVariantName},
    TemplateController, TemplateError,
};

impl TemplateController {
    pub fn resolve_template_and_variant(
        &self,
        input: &ParsedTemplateInput,
        variant_arg: &Option<Option<ValidVariantName>>,
    ) -> Result<(), TemplateError> {
        // let (raw_template, raw_variant) = input;
        // let template = self.service.get_existing_template(raw_template)?;

        // let variant = match variant_arg {
        //     // variant flag has value
        //     Some(Some(variant_arg_value)) => {
        //         if raw_variant.is_some() {
        //             self.view.warn_variant_ignored();
        //         }
        //         Some(
        //             self.service
        //                 .get_existing_variant(&template, variant_arg_value)?,
        //         )
        //     }
        //     // variant flag present with no value
        //     Some(None) => Some(self.select_variant(&template)?),
        //     // variant flag not used
        //     None => match raw_variant {
        //         Some(v) => Some(self.service.get_existing_variant(&template, &v)?),
        //         None => None,
        //     },
        // };

        // Ok((template, variant))
        Ok(())
    }
}
