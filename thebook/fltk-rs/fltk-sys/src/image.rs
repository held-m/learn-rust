/* automatically generated by rust-bindgen 0.56.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Image_draw(
        arg1: *mut Fl_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_width(arg1: *mut Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_height(arg1: *mut Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_delete(arg1: *mut Fl_Image);
}
extern "C" {
    pub fn Fl_Image_count(self_: *mut Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_data(self_: *mut Fl_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_Image_copy(self_: *mut Fl_Image) -> *mut Fl_Image;
}
extern "C" {
    pub fn Fl_Image_scale(
        self_: *mut Fl_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_fail(self_: *mut Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_data_w(self_: *const Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_data_h(self_: *const Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_d(self_: *const Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_ld(self_: *const Fl_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Image_inactive(self_: *mut Fl_Image);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_JPEG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_JPEG_Image_draw(
        arg1: *mut Fl_JPEG_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_width(arg1: *mut Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_height(arg1: *mut Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_delete(arg1: *mut Fl_JPEG_Image);
}
extern "C" {
    pub fn Fl_JPEG_Image_count(self_: *mut Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data(self_: *mut Fl_JPEG_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_JPEG_Image_copy(self_: *mut Fl_JPEG_Image) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_scale(
        self_: *mut Fl_JPEG_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_fail(self_: *mut Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data_w(self_: *const Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data_h(self_: *const Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_d(self_: *const Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_ld(self_: *const Fl_JPEG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_inactive(self_: *mut Fl_JPEG_Image);
}
extern "C" {
    pub fn Fl_JPEG_Image_new(filename: *const libc::c_char) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_from(data: *const libc::c_uchar) -> *mut Fl_JPEG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_PNG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_PNG_Image_draw(
        arg1: *mut Fl_PNG_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_width(arg1: *mut Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_height(arg1: *mut Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_delete(arg1: *mut Fl_PNG_Image);
}
extern "C" {
    pub fn Fl_PNG_Image_count(self_: *mut Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data(self_: *mut Fl_PNG_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_PNG_Image_copy(self_: *mut Fl_PNG_Image) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_scale(
        self_: *mut Fl_PNG_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_fail(self_: *mut Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data_w(self_: *const Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data_h(self_: *const Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_d(self_: *const Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_ld(self_: *const Fl_PNG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_inactive(self_: *mut Fl_PNG_Image);
}
extern "C" {
    pub fn Fl_PNG_Image_new(filename: *const libc::c_char) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_from(data: *const libc::c_uchar, size: libc::c_int) -> *mut Fl_PNG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_SVG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_SVG_Image_draw(
        arg1: *mut Fl_SVG_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_width(arg1: *mut Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_height(arg1: *mut Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_delete(arg1: *mut Fl_SVG_Image);
}
extern "C" {
    pub fn Fl_SVG_Image_count(self_: *mut Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data(self_: *mut Fl_SVG_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_SVG_Image_copy(self_: *mut Fl_SVG_Image) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_scale(
        self_: *mut Fl_SVG_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_fail(self_: *mut Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data_w(self_: *const Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data_h(self_: *const Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_d(self_: *const Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_ld(self_: *const Fl_SVG_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_inactive(self_: *mut Fl_SVG_Image);
}
extern "C" {
    pub fn Fl_SVG_Image_new(filename: *const libc::c_char) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_from(data: *const libc::c_char) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_normalize(self_: *mut Fl_SVG_Image);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_BMP_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_BMP_Image_draw(
        arg1: *mut Fl_BMP_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_width(arg1: *mut Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_height(arg1: *mut Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_delete(arg1: *mut Fl_BMP_Image);
}
extern "C" {
    pub fn Fl_BMP_Image_count(self_: *mut Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data(self_: *mut Fl_BMP_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_BMP_Image_copy(self_: *mut Fl_BMP_Image) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_scale(
        self_: *mut Fl_BMP_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_fail(self_: *mut Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data_w(self_: *const Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data_h(self_: *const Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_d(self_: *const Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_ld(self_: *const Fl_BMP_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_inactive(self_: *mut Fl_BMP_Image);
}
extern "C" {
    pub fn Fl_BMP_Image_new(filename: *const libc::c_char) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_from(data: *const libc::c_uchar) -> *mut Fl_BMP_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_GIF_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_GIF_Image_draw(
        arg1: *mut Fl_GIF_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_width(arg1: *mut Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_height(arg1: *mut Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_delete(arg1: *mut Fl_GIF_Image);
}
extern "C" {
    pub fn Fl_GIF_Image_count(self_: *mut Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data(self_: *mut Fl_GIF_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_GIF_Image_copy(self_: *mut Fl_GIF_Image) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_scale(
        self_: *mut Fl_GIF_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_fail(self_: *mut Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data_w(self_: *const Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data_h(self_: *const Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_d(self_: *const Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_ld(self_: *const Fl_GIF_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_inactive(self_: *mut Fl_GIF_Image);
}
extern "C" {
    pub fn Fl_GIF_Image_new(filename: *const libc::c_char) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_from(data: *const libc::c_uchar) -> *mut Fl_GIF_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Pixmap {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Pixmap_draw(
        arg1: *mut Fl_Pixmap,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Pixmap_width(arg1: *mut Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_height(arg1: *mut Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_delete(arg1: *mut Fl_Pixmap);
}
extern "C" {
    pub fn Fl_Pixmap_count(self_: *mut Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data(self_: *mut Fl_Pixmap) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_Pixmap_copy(self_: *mut Fl_Pixmap) -> *mut Fl_Pixmap;
}
extern "C" {
    pub fn Fl_Pixmap_scale(
        self_: *mut Fl_Pixmap,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Pixmap_fail(self_: *mut Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data_w(self_: *const Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_data_h(self_: *const Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_d(self_: *const Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_ld(self_: *const Fl_Pixmap) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Pixmap_inactive(self_: *mut Fl_Pixmap);
}
extern "C" {
    pub fn Fl_Pixmap_new(D: *const *const libc::c_char) -> *mut Fl_Pixmap;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_XPM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_XPM_Image_draw(
        arg1: *mut Fl_XPM_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_XPM_Image_width(arg1: *mut Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_height(arg1: *mut Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_delete(arg1: *mut Fl_XPM_Image);
}
extern "C" {
    pub fn Fl_XPM_Image_count(self_: *mut Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data(self_: *mut Fl_XPM_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_XPM_Image_copy(self_: *mut Fl_XPM_Image) -> *mut Fl_XPM_Image;
}
extern "C" {
    pub fn Fl_XPM_Image_scale(
        self_: *mut Fl_XPM_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_XPM_Image_fail(self_: *mut Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data_w(self_: *const Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_data_h(self_: *const Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_d(self_: *const Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_ld(self_: *const Fl_XPM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XPM_Image_inactive(self_: *mut Fl_XPM_Image);
}
extern "C" {
    pub fn Fl_XPM_Image_new(filename: *const libc::c_char) -> *mut Fl_XPM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_XBM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_XBM_Image_draw(
        arg1: *mut Fl_XBM_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_XBM_Image_width(arg1: *mut Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_height(arg1: *mut Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_delete(arg1: *mut Fl_XBM_Image);
}
extern "C" {
    pub fn Fl_XBM_Image_count(self_: *mut Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data(self_: *mut Fl_XBM_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_XBM_Image_copy(self_: *mut Fl_XBM_Image) -> *mut Fl_XBM_Image;
}
extern "C" {
    pub fn Fl_XBM_Image_scale(
        self_: *mut Fl_XBM_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_XBM_Image_fail(self_: *mut Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data_w(self_: *const Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_data_h(self_: *const Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_d(self_: *const Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_ld(self_: *const Fl_XBM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_XBM_Image_inactive(self_: *mut Fl_XBM_Image);
}
extern "C" {
    pub fn Fl_XBM_Image_new(filename: *const libc::c_char) -> *mut Fl_XBM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_PNM_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_PNM_Image_draw(
        arg1: *mut Fl_PNM_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_PNM_Image_width(arg1: *mut Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_height(arg1: *mut Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_delete(arg1: *mut Fl_PNM_Image);
}
extern "C" {
    pub fn Fl_PNM_Image_count(self_: *mut Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data(self_: *mut Fl_PNM_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_PNM_Image_copy(self_: *mut Fl_PNM_Image) -> *mut Fl_PNM_Image;
}
extern "C" {
    pub fn Fl_PNM_Image_scale(
        self_: *mut Fl_PNM_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_PNM_Image_fail(self_: *mut Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data_w(self_: *const Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_data_h(self_: *const Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_d(self_: *const Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_ld(self_: *const Fl_PNM_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_PNM_Image_inactive(self_: *mut Fl_PNM_Image);
}
extern "C" {
    pub fn Fl_PNM_Image_new(filename: *const libc::c_char) -> *mut Fl_PNM_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Tiled_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Tiled_Image_draw(
        arg1: *mut Fl_Tiled_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Tiled_Image_width(arg1: *mut Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_height(arg1: *mut Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_delete(arg1: *mut Fl_Tiled_Image);
}
extern "C" {
    pub fn Fl_Tiled_Image_count(self_: *mut Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data(self_: *mut Fl_Tiled_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_Tiled_Image_copy(self_: *mut Fl_Tiled_Image) -> *mut Fl_Tiled_Image;
}
extern "C" {
    pub fn Fl_Tiled_Image_scale(
        self_: *mut Fl_Tiled_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Tiled_Image_fail(self_: *mut Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data_w(self_: *const Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_data_h(self_: *const Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_d(self_: *const Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_ld(self_: *const Fl_Tiled_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Tiled_Image_inactive(self_: *mut Fl_Tiled_Image);
}
extern "C" {
    pub fn Fl_Tiled_Image_new(
        i: *mut Fl_Image,
        w: libc::c_int,
        h: libc::c_int,
    ) -> *mut Fl_Tiled_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_RGB_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_RGB_Image_draw(
        arg1: *mut Fl_RGB_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_width(arg1: *mut Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_height(arg1: *mut Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_delete(arg1: *mut Fl_RGB_Image);
}
extern "C" {
    pub fn Fl_RGB_Image_count(self_: *mut Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data(self_: *mut Fl_RGB_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_RGB_Image_copy(self_: *mut Fl_RGB_Image) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_scale(
        self_: *mut Fl_RGB_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_fail(self_: *mut Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data_w(self_: *const Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data_h(self_: *const Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_d(self_: *const Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_ld(self_: *const Fl_RGB_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_inactive(self_: *mut Fl_RGB_Image);
}
extern "C" {
    pub fn Fl_RGB_Image_new(
        bits: *const libc::c_uchar,
        W: libc::c_int,
        H: libc::c_int,
        depth: libc::c_int,
        ld: libc::c_int,
    ) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_from_data(
        bits: *const libc::c_uchar,
        W: libc::c_int,
        H: libc::c_int,
        depth: libc::c_int,
        ld: libc::c_int,
    ) -> *mut Fl_RGB_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Shared_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Shared_Image_draw(
        arg1: *mut Fl_Shared_Image,
        X: libc::c_int,
        Y: libc::c_int,
        W: libc::c_int,
        H: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Shared_Image_width(arg1: *mut Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_height(arg1: *mut Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_delete(arg1: *mut Fl_Shared_Image);
}
extern "C" {
    pub fn Fl_Shared_Image_count(self_: *mut Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data(self_: *mut Fl_Shared_Image) -> *const *const libc::c_char;
}
extern "C" {
    pub fn Fl_Shared_Image_copy(self_: *mut Fl_Shared_Image) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_scale(
        self_: *mut Fl_Shared_Image,
        width: libc::c_int,
        height: libc::c_int,
        proportional: libc::c_int,
        can_expand: libc::c_int,
    );
}
extern "C" {
    pub fn Fl_Shared_Image_fail(self_: *mut Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data_w(self_: *const Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_data_h(self_: *const Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_d(self_: *const Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_ld(self_: *const Fl_Shared_Image) -> libc::c_int;
}
extern "C" {
    pub fn Fl_Shared_Image_inactive(self_: *mut Fl_Shared_Image);
}
extern "C" {
    pub fn Fl_Shared_Image_get(
        name: *const libc::c_char,
        W: libc::c_int,
        H: libc::c_int,
    ) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_Shared_Image_from_rgb(
        rgb: *mut Fl_RGB_Image,
        own_it: libc::c_int,
    ) -> *mut Fl_Shared_Image;
}
extern "C" {
    pub fn Fl_register_images();
}