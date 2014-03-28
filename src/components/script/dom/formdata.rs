/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use dom::bindings::error::{Fallible};
use dom::bindings::codegen::BindingDeclarations::FormDataBinding;
use dom::bindings::js::{JS, JSRef};
use dom::blob::Blob;
use dom::htmlformelement::HTMLFormElement;
use dom::window::Window;
use servo_util::str::DOMString;

use collections::hashmap::HashMap;

#[deriving(Encodable)]
pub enum FormDatum {
    StringData(DOMString),
    BlobData { blob: JS<Blob>, name: DOMString }
}

#[deriving(Encodable)]
pub struct FormData {
    pub data: HashMap<DOMString, FormDatum>,
    pub reflector_: Reflector,
    pub window: JS<Window>,
    pub form: Option<JS<HTMLFormElement>>
}

impl FormData {
    pub fn new_inherited(form: Option<JSRef<HTMLFormElement>>, window: JS<Window>) -> FormData {
        FormData {
            data: HashMap::new(),
            reflector_: Reflector::new(),
            window: window,
            form: form.map(|form| form.unrooted())
        }
    }

    pub fn new(form: Option<JSRef<HTMLFormElement>>, window: &JSRef<Window>) -> JS<FormData> {
        reflect_dom_object(~FormData::new_inherited(form, window.unrooted()), window, FormDataBinding::Wrap)
    }

    pub fn Constructor(window: &JSRef<Window>, form: Option<JSRef<HTMLFormElement>>)
                       -> Fallible<JS<FormData>> {
        Ok(FormData::new(form, window))
    }

    pub fn Append(&mut self, name: DOMString, value: &JSRef<Blob>, filename: Option<DOMString>) {
        let blob = BlobData {
            blob: value.unrooted(),
            name: filename.unwrap_or(~"default")
        };
        self.data.insert(name.clone(), blob);
    }

    pub fn Append_(&mut self, name: DOMString, value: DOMString) {
        self.data.insert(name, StringData(value));
    }
}

impl Reflectable for FormData {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        &self.reflector_
    }

    fn mut_reflector<'a>(&'a mut self) -> &'a mut Reflector {
        &mut self.reflector_
    }
}
