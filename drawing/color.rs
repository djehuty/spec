use tester::*;

#[path="../tester.rs"]
mod tester;

mod drawing {
  extern mod color;
}

describe!("Color", {
  test!("hsla_to_rgba", {
    should!("retain alpha value", {
      let color = drawing::color::Color::hsla_to_rgba(1.0, 0.3, 0.2, 0.5);
      must!(color.alpha() eq 0.5)
    })

    should!("determine the correct red value", {
      let color = drawing::color::Color::hsla_to_rgba(0.6111111, 0.72, 0.54, 1.0);
      must!(color.red() near 0.2088 within 0.002)
    })

    should!("determine the correct green value", {
      let color = drawing::color::Color::hsla_to_rgba(0.611111111, 0.72, 0.54, 1.0);
      must!(color.green() near 0.4296 within 0.002)
    })

    should!("determine the correct blue value", {
      let color = drawing::color::Color::hsla_to_rgba(0.611111111, 0.72, 0.54, 1.0);
      must!(color.blue() near 0.8705 within 0.002)
    })
  })

  test!("rgba_to_hsla", {
    should!("retain alpha value", {
      let color = drawing::color::Color::rgba_to_hsla(1.0, 0.3, 0.2, 0.5);
      must!(color.alpha() eq 0.5)
    })

    should!("determine the correct red value", {
      let color = drawing::color::Color::rgba_to_hsla(0.2088, 0.4296, 0.8705, 1.0);
      must!(color.hue() near 0.61111 within 0.002)
    })

    should!("determine the correct green value", {
      let color = drawing::color::Color::rgba_to_hsla(0.2088, 0.4296, 0.8705, 1.0);
      must!(color.saturation() near 0.72 within 0.002)
    })

    should!("determine the correct blue value", {
      let color = drawing::color::Color::rgba_to_hsla(0.2088, 0.4296, 0.8705, 1.0);
      must!(color.luminance() near 0.54 within 0.002)
    })
  })
})
