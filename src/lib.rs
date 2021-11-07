use std::ffi::c_void;

use objc::{msg_send, sel, sel_impl};
use objc_derive::selector_export;
use tao_foundation::{Class, GetObjcObject, NSArray, NSDictionary, NSString, NSUInteger, NSURL, id};

#[repr(transparent)]
#[derive(Clone)]
pub struct UIDocumentPickerViewController(pub id);
impl std::ops::Deref for UIDocumentPickerViewController {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIDocumentPickerViewController {}
impl UIDocumentPickerViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIDocumentPickerViewController), alloc) })
    }
}

impl UIDocumentPickerViewController {
    #[selector_export("initForOpeningContentTypes:")]
    pub fn init_for_opening_content_types(&self, content_types: NSArray) -> UIDocumentPickerViewController;

    #[selector_export("setDelegate:")]
    pub fn set_delegate(&self, delegate: id);
}

impl GetObjcObject for UIDocumentPickerViewController {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UIApplication(pub id);
impl std::ops::Deref for UIApplication {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIApplication {}
impl UIApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIApplication), alloc) })
    }
}

impl UIApplication {
    #[selector_export(UIApplication, "sharedApplication")]
    pub fn shared_application() -> UIApplication;

    #[selector_export("keyWindow")]
    pub fn key_window(&self) -> UIWindow;

    #[selector_export("openURL:options:completionHandler:")]
    pub fn open_url_options_completion_handler(&self, url: NSURL, options: NSDictionary, completion_handler: id);
}

impl GetObjcObject for UIApplication {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UIWindow(pub id);
impl std::ops::Deref for UIWindow {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIWindow {}
impl UIWindow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIWindow), alloc) })
    }
}

impl UIWindow {
    #[selector_export("rootViewController")]
    pub fn root_view_controller(&self) -> UIViewController;

    #[selector_export("setRootViewController:")]
    pub fn set_root_view_controller(&self, root_view_controller: UIViewController);

    #[selector_export("initWithWindowScene:")]
    pub fn init_with_window_scene(&self, window_scene: id) -> Self;

    #[selector_export("makeKeyAndVisible")]
    pub fn make_key_and_visible(&self);
}

impl GetObjcObject for UIWindow {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UIViewController(pub id);
impl std::ops::Deref for UIViewController {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIViewController {}
impl UIViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIViewController), alloc) })
    }
}

impl UIViewController {
    #[selector_export("presentViewController:animated:completion:")]
    pub fn present_view_controller_animated_completion(&self, view_controller_to_present: id, animated: bool, completion: *const c_void);

    #[selector_export("view")]
    pub fn view(&self) -> UIView;
}

impl GetObjcObject for UIViewController {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UIDocumentInteractionController(pub id);
impl std::ops::Deref for UIDocumentInteractionController {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIDocumentInteractionController {}
impl UIDocumentInteractionController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIDocumentInteractionController), alloc) })
    }
}

impl UIDocumentInteractionController {
    #[selector_export(UIDocumentInteractionController, "interactionControllerWithURL:")]
    pub fn interaction_controller_with_url(url: NSURL) -> Self;

    #[selector_export("setDelegate:")]
    pub fn set_delegate(&self, delegate: id);

    #[selector_export("presentPreviewAnimated:")]
    pub fn present_preview_animated(&self, animated: bool) -> bool;
}

impl GetObjcObject for UIDocumentInteractionController {
    fn objc_object(&self) -> id {
        self.0
    }
}


#[repr(transparent)]
#[derive(Clone)]
pub struct UISceneConfiguration(pub id);
impl std::ops::Deref for UISceneConfiguration {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UISceneConfiguration {}
impl UISceneConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UISceneConfiguration), alloc) })
    }
}

impl UISceneConfiguration {
    #[selector_export(UISceneConfiguration, "configurationWithName:sessionRole:")]
    pub fn configuration_with_name_session_role(name: NSString, session_role: NSString) -> Self;

    #[selector_export("setDelegateClass:")]
    pub fn set_delegate_class(&self, delegate_class: Class);

    #[selector_export("hash")]
    pub fn hash(&self) -> NSUInteger;
}

impl GetObjcObject for UISceneConfiguration {
    fn objc_object(&self) -> id {
        self.0
    }
}



#[repr(transparent)]
#[derive(Clone)]
pub struct UIView(pub id);
impl std::ops::Deref for UIView {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIView {}
impl UIView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIView), alloc) })
    }
}

impl UIView {
    #[selector_export("setBackgroundColor:")]
    pub fn set_background_color(&self, background_color: UIColor);
}

impl GetObjcObject for UIView {
    fn objc_object(&self) -> id {
        self.0
    }
}



#[repr(transparent)]
#[derive(Clone)]
pub struct UIColor(pub id);
impl std::ops::Deref for UIColor {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for UIColor {}
impl UIColor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(UIColor), alloc) })
    }
}

impl UIColor {
    #[selector_export(UIColor, "systemBlueColor")]
    pub fn system_blue_color() -> Self;

    #[selector_export(UIColor, "whiteColor")]
    pub fn white_color() -> Self;
}

impl GetObjcObject for UIColor {
    fn objc_object(&self) -> id {
        self.0
    }
}