use std::ops::Deref;

use glam::{Vec4, Vec4Swizzles};

use crate::{
    L8Surface, L32Surface, La8Surface, La32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface,
    Zrgb8Surface,
};

mod rgba32;
mod zrgb8;

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

const fn f32_to_u8(v: f32) -> u8 {
    (v * 256.) as u8
}

macro_rules! impl_from_surface {
    ($from:ty, $to:tt, $converter:expr) => {
        impl From<&$from> for $to {
            fn from(value: &$from) -> Self {
                let buffer = value
                    .buffer
                    .iter()
                    .map(|pixel| $converter(*pixel))
                    .collect();
                Self {
                    size: value.size,
                    buffer,
                    destination_rect: value.destination_rect,
                    blit_area: value.blit_area,
                }
            }
        }
    };
}

// L8
const fn l8_to_zrgb(pixel: u8) -> u32 {
    let pixel = pixel as u32;
    ((0 | pixel << 24) | pixel << 16) | pixel << 8
}
impl_from_surface!(L8Surface, La8Surface, |p| [p, 255]);
impl_from_surface!(L8Surface, L32Surface, u8_to_f32);
impl_from_surface!(L8Surface, La32Surface, |p| [u8_to_f32(p), 1.]);
impl_from_surface!(L8Surface, Rgb8Surface, |p| [p; 3]);
impl_from_surface!(L8Surface, Rgba8Surface, |p| [p, p, p, 255]);
impl_from_surface!(L8Surface, Zrgb8Surface, l8_to_zrgb);
impl_from_surface!(L8Surface, Rgba32Surface, |p| {
    let pixel = u8_to_f32(p);
    Vec4::new(pixel, pixel, pixel, 1.)
});

// La8
type La8 = [u8; 2];
impl_from_surface!(La8Surface, L8Surface, |p: La8| p[0]);
impl_from_surface!(La8Surface, L32Surface, |p: La8| u8_to_f32(p[0]));
impl_from_surface!(La8Surface, La32Surface, |p: La8| [
    u8_to_f32(p[0]),
    u8_to_f32(p[1])
]);
impl_from_surface!(La8Surface, Rgb8Surface, |p: La8| [p[0]; 3]);
impl_from_surface!(La8Surface, Rgba8Surface, |p: La8| {
    let c = p[0];
    [c, c, c, 255]
});
impl_from_surface!(La8Surface, Zrgb8Surface, |p: La8| l8_to_zrgb(p[0]));
impl_from_surface!(La8Surface, Rgba32Surface, |p: La8| {
    let c = u8_to_f32(p[0]);
    let a = u8_to_f32(p[1]);
    Vec4::new(c, c, c, a)
});

// La32
impl_from_surface!(L32Surface, L8Surface, f32_to_u8);
impl_from_surface!(L32Surface, La8Surface, |p| [f32_to_u8(p), 255]);
impl_from_surface!(L32Surface, La32Surface, |p| [p, 1.]);
impl_from_surface!(L32Surface, Rgb8Surface, |p| {
    let c = f32_to_u8(p);
    [c; 3]
});
impl_from_surface!(L32Surface, Rgba8Surface, |p| {
    let c = f32_to_u8(p);
    [c, c, c, 255]
});
impl_from_surface!(L32Surface, Zrgb8Surface, |p| l8_to_zrgb(f32_to_u8(p)));
impl_from_surface!(L32Surface, Rgba32Surface, |p| Vec4::new(p, p, p, 1.));

// La32
type La32 = [f32; 2];
impl_from_surface!(La32Surface, L8Surface, |p: La32| f32_to_u8(p[0]));
impl_from_surface!(La32Surface, La8Surface, |p: La32| [
    f32_to_u8(p[0]),
    f32_to_u8(p[1])
]);
impl_from_surface!(La32Surface, L32Surface, |p: La32| p[0]);
impl_from_surface!(La32Surface, Rgb8Surface, |p: La32| [f32_to_u8(p[0]); 3]);
impl_from_surface!(La32Surface, Rgba8Surface, |p: La32| {
    let c = f32_to_u8(p[0]);
    let a = f32_to_u8(p[1]);
    [c, c, c, a]
});
impl_from_surface!(La32Surface, Zrgb8Surface, |p: La32| l8_to_zrgb(f32_to_u8(
    p[0]
)));
impl_from_surface!(La32Surface, Rgba32Surface, |p: La32| {
    let c = p[0];
    let a = p[1];
    Vec4::new(c, c, c, a)
});

// Rgb8
const fn grayscale(r: u8, g: u8, b: u8) -> f32 {
    (u8_to_f32(r) + u8_to_f32(g) + u8_to_f32(b)) / 3.
}

const fn rgb8_to_l8<const N: usize>(p: [u8; N]) -> u8 {
    f32_to_u8(rgb8_to_l32(p))
}

const fn rgb8_to_l32<const N: usize>(p: [u8; N]) -> f32 {
    grayscale(p[0], p[1], p[2])
}
const fn rgb8_to_zrgb8<const N: usize>(p: [u8; N]) -> u32 {
    u32::from_le_bytes([0, p[0], p[1], p[2]])
}
type Rgb8 = [u8; 3];
impl_from_surface!(Rgb8Surface, L8Surface, rgb8_to_l8);
impl_from_surface!(Rgb8Surface, La8Surface, |p| [rgb8_to_l8(p), 255]);
impl_from_surface!(Rgb8Surface, L32Surface, rgb8_to_l32);
impl_from_surface!(Rgb8Surface, La32Surface, |p| [rgb8_to_l32(p), 1.]);
impl_from_surface!(Rgb8Surface, Rgba8Surface, |p: Rgb8| [p[0], p[1], p[2], 255]);
impl_from_surface!(Rgb8Surface, Zrgb8Surface, rgb8_to_zrgb8);
impl_from_surface!(Rgb8Surface, Rgba32Surface, |p: Rgb8| Vec4::new(
    u8_to_f32(p[0]),
    u8_to_f32(p[1]),
    u8_to_f32(p[2]),
    1.,
));

// Rgba8
type Rgba8 = [u8; 4];
impl_from_surface!(Rgba8Surface, L8Surface, rgb8_to_l8);
impl_from_surface!(Rgba8Surface, La8Surface, |p| [rgb8_to_l8(p), 255]);
impl_from_surface!(Rgba8Surface, L32Surface, rgb8_to_l32);
impl_from_surface!(Rgba8Surface, La32Surface, |p| [
    rgb8_to_l32(p),
    u8_to_f32(p[3])
]);
impl_from_surface!(Rgba8Surface, Rgb8Surface, |p: Rgba8| [p[0], p[1], p[2]]);
impl_from_surface!(Rgba8Surface, Zrgb8Surface, rgb8_to_zrgb8);
impl_from_surface!(Rgba8Surface, Rgba32Surface, |p: Rgba8| Vec4::new(
    u8_to_f32(p[0]),
    u8_to_f32(p[1]),
    u8_to_f32(p[2]),
    u8_to_f32(p[3])
));

// Zrgb8
const fn zrgb8_to_l8(p: u32) -> u8 {
    f32_to_u8(zrgb8_to_l32(p))
}

const fn zrgb8_to_l32(p: u32) -> f32 {
    let p = p.to_le_bytes();
    grayscale(p[1], p[2], p[3])
}

impl_from_surface!(Zrgb8Surface, L8Surface, zrgb8_to_l8);
impl_from_surface!(Zrgb8Surface, La8Surface, |p| [zrgb8_to_l8(p), 255]);
impl_from_surface!(Zrgb8Surface, L32Surface, zrgb8_to_l32);
impl_from_surface!(Zrgb8Surface, La32Surface, |p| [zrgb8_to_l32(p), 1.]);
impl_from_surface!(Zrgb8Surface, Rgb8Surface, |p: u32| {
    let p = p.to_le_bytes();
    [p[1], p[2], p[3]]
});
impl_from_surface!(Zrgb8Surface, Rgba8Surface, |p: u32| {
    let p = p.to_le_bytes();
    [p[1], p[2], p[3], 255]
});
impl_from_surface!(Zrgb8Surface, Rgba32Surface, |p: u32| {
    let p = p.to_le_bytes();
    Vec4::new(u8_to_f32(p[1]), u8_to_f32(p[2]), u8_to_f32(p[3]), 1.)
});

// Rgba32
fn rgba32_to_l32(p: Vec4) -> f32 {
    p.xyz().element_sum() / 3.
}
impl_from_surface!(Rgba32Surface, L8Surface, |p| f32_to_u8(rgba32_to_l32(p)));
impl_from_surface!(Rgba32Surface, La8Surface, |p: Vec4| {
    let p = p.deref();
    let c = (p.x + p.y + p.z) / 3.;
    [f32_to_u8(c), f32_to_u8(p.w)]
});
impl_from_surface!(Rgba32Surface, L32Surface, rgba32_to_l32);
impl_from_surface!(Rgba32Surface, La32Surface, |p: Vec4| {
    let p = p.deref();
    let c = (p.x + p.y + p.z) / 3.;
    [c, p.w]
});
impl_from_surface!(Rgba32Surface, Rgb8Surface, |p: Vec4| {
    let p = p.deref();
    [f32_to_u8(p.x), f32_to_u8(p.y), f32_to_u8(p.z)]
});
impl_from_surface!(Rgba32Surface, Rgba8Surface, |p: Vec4| {
    {
        let p = p.deref();
        [
            f32_to_u8(p.x),
            f32_to_u8(p.y),
            f32_to_u8(p.z),
            f32_to_u8(p.w),
        ]
    }
});
impl_from_surface!(Rgba32Surface, Zrgb8Surface, |p: Vec4| {
    let p = p.deref();
    u32::from_le_bytes([0, f32_to_u8(p.x), f32_to_u8(p.y), f32_to_u8(p.z)])
});

#[cfg(test)]
mod tests {
    #[test]
    fn test_u32() {
        let gray = 120u8;
        let a = u32::from_le_bytes([0, gray, gray, gray]);
        let mut b = 0u32;
        let gray = gray as u32;
        b |= gray << 24;
        b |= gray << 16;
        b |= gray << 8;
        let c = ((0 | gray << 24) | gray << 16) | gray << 8;
        assert_eq!(a, b);
        assert_eq!(a, c);
    }
}
