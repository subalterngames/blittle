use crate::{Rgb8Surface, Rgba8Surface, Rgba32Surface};
use glam::Vec4;
use std::ops::Deref;

const fn u8_to_f32(v: u8) -> f32 {
    const U8_TO_F32: [f32; 256] = [
        0.0, 0.0039216, 0.0078431, 0.0117647, 0.0156863, 0.0196078, 0.0235294, 0.027451, 0.0313725,
        0.0352941, 0.0392157, 0.0431373, 0.0470588, 0.0509804, 0.054902, 0.0588235, 0.0627451,
        0.0666667, 0.0705882, 0.0745098, 0.0784314, 0.0823529, 0.0862745, 0.0901961, 0.0941176,
        0.0980392, 0.1019608, 0.1058824, 0.1098039, 0.1137255, 0.1176471, 0.1215686, 0.1254902,
        0.1294118, 0.1333333, 0.1372549, 0.1411765, 0.145098, 0.1490196, 0.1529412, 0.1568627,
        0.1607843, 0.1647059, 0.1686275, 0.172549, 0.1764706, 0.1803922, 0.1843137, 0.1882353,
        0.1921569, 0.1960784, 0.2, 0.2039216, 0.2078431, 0.2117647, 0.2156863, 0.2196078,
        0.2235294, 0.227451, 0.2313725, 0.2352941, 0.2392157, 0.2431373, 0.2470588, 0.2509804,
        0.254902, 0.2588235, 0.2627451, 0.2666667, 0.2705882, 0.2745098, 0.2784314, 0.2823529,
        0.2862745, 0.2901961, 0.2941176, 0.2980392, 0.3019608, 0.3058824, 0.3098039, 0.3137255,
        0.3176471, 0.3215686, 0.3254902, 0.3294118, 0.3333333, 0.3372549, 0.3411765, 0.345098,
        0.3490196, 0.3529412, 0.3568627, 0.3607843, 0.3647059, 0.3686275, 0.372549, 0.3764706,
        0.3803922, 0.3843137, 0.3882353, 0.3921569, 0.3960784, 0.4, 0.4039216, 0.4078431,
        0.4117647, 0.4156863, 0.4196078, 0.4235294, 0.427451, 0.4313725, 0.4352941, 0.4392157,
        0.4431373, 0.4470588, 0.4509804, 0.454902, 0.4588235, 0.4627451, 0.4666667, 0.4705882,
        0.4745098, 0.4784314, 0.4823529, 0.4862745, 0.4901961, 0.4941176, 0.4980392, 0.5019608,
        0.5058824, 0.5098039, 0.5137255, 0.5176471, 0.5215686, 0.5254902, 0.5294118, 0.5333333,
        0.5372549, 0.5411765, 0.545098, 0.5490196, 0.5529412, 0.5568627, 0.5607843, 0.5647059,
        0.5686275, 0.572549, 0.5764706, 0.5803922, 0.5843137, 0.5882353, 0.5921569, 0.5960784, 0.6,
        0.6039216, 0.6078431, 0.6117647, 0.6156863, 0.6196078, 0.6235294, 0.627451, 0.6313725,
        0.6352941, 0.6392157, 0.6431373, 0.6470588, 0.6509804, 0.654902, 0.6588235, 0.6627451,
        0.6666667, 0.6705882, 0.6745098, 0.6784314, 0.6823529, 0.6862745, 0.6901961, 0.6941176,
        0.6980392, 0.7019608, 0.7058824, 0.7098039, 0.7137255, 0.7176471, 0.7215686, 0.7254902,
        0.7294118, 0.7333333, 0.7372549, 0.7411765, 0.745098, 0.7490196, 0.7529412, 0.7568627,
        0.7607843, 0.7647059, 0.7686275, 0.772549, 0.7764706, 0.7803922, 0.7843137, 0.7882353,
        0.7921569, 0.7960784, 0.8, 0.8039216, 0.8078431, 0.8117647, 0.8156863, 0.8196078,
        0.8235294, 0.827451, 0.8313725, 0.8352941, 0.8392157, 0.8431373, 0.8470588, 0.8509804,
        0.854902, 0.8588235, 0.8627451, 0.8666667, 0.8705882, 0.8745098, 0.8784314, 0.8823529,
        0.8862745, 0.8901961, 0.8941176, 0.8980392, 0.9019608, 0.9058824, 0.9098039, 0.9137255,
        0.9176471, 0.9215686, 0.9254902, 0.9294118, 0.9333333, 0.9372549, 0.9411765, 0.945098,
        0.9490196, 0.9529412, 0.9568627, 0.9607843, 0.9647059, 0.9686275, 0.972549, 0.9764706,
        0.9803922, 0.9843137, 0.9882353, 0.9921569, 0.9960784, 1.0,
    ];

    U8_TO_F32[v as usize]
}

macro_rules! bytes_to_floats {
    ($self:ident) => {
        /// Creates a new surface, converting pixel values, where the alpha channel value is always 1.
        pub fn get_rgba32(&$self) -> Rgba32Surface {
            let buffer = $self
                .buffer
                .iter()
                .map(|pixel| Self::pixel_to_rgba32(pixel))
                .collect();
            Rgba32Surface {
                size: $self.size,
                buffer,
                destination_rect: $self.destination_rect,
                blit_area: $self.blit_area,
            }
        }

        /// Copy data into `other`, converting pixel values.
        pub fn set_rgba32(&$self, other: &mut Rgba32Surface) {
            $self.buffer
                .iter()
                .zip(other.buffer.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = Self::pixel_to_rgba32(src);
                });
            other.size = $self.size;
            other.destination_rect = $self.destination_rect;
            other.blit_area = $self.blit_area;
        }
    };
}

macro_rules! floats_to_bytes {
    ($self:ident, $get:ident, $set:ident, $pixel:ident, $dest:tt) => {
        /// Creates a new surface, converting pixel values.
        pub fn $get(&$self) -> $dest {
            let buffer = $self
                .buffer
                .iter()
                .map(|pixel| Self::$pixel(pixel))
                .collect();
            $dest {
                size: $self.size,
                buffer,
                destination_rect: $self.destination_rect,
                blit_area: $self.blit_area,
            }
        }

        /// Copy data into `other`, converting pixel values.
        pub fn $set(&$self, other: &mut $dest) {
            $self.buffer
                .iter()
                .zip(other.buffer.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = Self::$pixel(src);
                });
            other.size = $self.size;
            other.destination_rect = $self.destination_rect;
            other.blit_area = $self.blit_area;
        }
    };
}

impl Rgb8Surface {
    /// Convert an RGB8 pixel to an RGBA32 pixel.
    pub const fn pixel_to_rgba32(pixel: &[u8; 3]) -> Vec4 {
        Vec4::new(
            u8_to_f32(pixel[0]),
            u8_to_f32(pixel[1]),
            u8_to_f32(pixel[2]),
            1.,
        )
    }

    bytes_to_floats!(self);
}

impl Rgba8Surface {
    /// Convert an RGBA8 pixel to an RGBA32 pixel.
    pub const fn pixel_to_rgba32(pixel: &[u8; 4]) -> Vec4 {
        Vec4::new(
            u8_to_f32(pixel[0]),
            u8_to_f32(pixel[1]),
            u8_to_f32(pixel[2]),
            u8_to_f32(pixel[3]),
        )
    }

    bytes_to_floats!(self);
}

impl Rgba32Surface {
    /// Convert an RGBA32 pixel to an RGBA32 pixel.
    #[inline]
    pub fn pixel_to_rgb8(pixel: &Vec4) -> [u8; 3] {
        let color = pixel * 256.;
        let color = color.deref();
        [color.x as u8, color.y as u8, color.z as u8]
    }

    /// Convert an RGBA32 pixel to an RGBA8 pixel.
    #[inline]
    pub fn pixel_to_rgba8(color: &Vec4) -> [u8; 4] {
        let color = color * 256.;
        let color = color.deref();
        [color.x as u8, color.y as u8, color.z as u8, color.w as u8]
    }

    floats_to_bytes!(self, get_rgb8, set_rgb8, pixel_to_rgb8, Rgb8Surface);

    floats_to_bytes!(self, get_rgba8, set_rgba8, pixel_to_rgba8, Rgba8Surface);
}
