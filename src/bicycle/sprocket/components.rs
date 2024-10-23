use std::f32::consts::PI;

use bevy::math::DVec2;

pub struct SprocketOptions {
    pub(crate) size: f32,
    pub(crate) teeth: u32,
}

pub struct Sprocket {
    options: SprocketOptions,
}

impl Sprocket {
    pub(crate) fn new(options: SprocketOptions) -> Self {
        Self { options }
    }

    fn get_pitch(self) {}

    fn get_roller_diameter(self) {}

    fn invert_x(p: (f32, f32)) -> (f32, f32) {
        (-p.0, p.1)
    }

    pub(crate) fn get_geometry(self) -> Vec<DVec2> {
        self.effect()
    }

    fn effect(self) -> Vec<DVec2> {
        let mut points: Vec<DVec2> = vec![];

        let p = 12.70;
        let n = self.options.teeth;
        let pd = p / f32::sin(PI / n as f32);
        let pr = pd / 2_f32;
        let dr = 7.77;
        let ds = 1.0005 * dr + 0.0762;
        let r = ds / 2_f32; // Seating Curve Radius
        let a = f32::to_radians(35.0 + 60.0 / n as f32);
        let b = f32::to_radians(18.0 - 56.0 / n as f32);
        let ac = 0.8 * dr;
        let m = ac * f32::cos(a);
        let t = ac * f32::sin(a);
        let e = 1.3025 * dr + 0.0381;
        let ab = 1.4 * dr;
        let w = ab * f32::cos(PI / n as f32);
        let v = ab * f32::sin(PI / n as f32);
        let f = dr * (0.8 * f32::cos(b) + 1.4 * f32::cos(17.0 - 64.0 / n as f32) - 1.3025) - 0.0381;
        let t_inc = 2.0 * PI / n as f32;

        let mut thetas: Vec<f32> = vec![];

        for x in 0..n {
            let theta = x as f32 * t_inc;
            thetas.push(theta);
        }

        for theta in thetas {
            println!("THETA: {}", theta);

            // Seating curve center
            let seat_c = (0.0, -pr);
            println!("SEATING CURVE CENTER: {:?}", seat_c);

            // Transitional curve center
            let c = (m, -pr - t);

            // Calculate line cx, angle A from x axis
            // Y = mX + b
            let cx_m = -f32::tan(a);
            let cx_b = c.1 - cx_m * c.0;

            // Calculate intersection of cx with circle S to get point x
            // http://math.stackexchange.com/questions/228841/how-do-i-calculate-the-intersections-of-a-straight-line-and-a-circle
            let q_a = cx_m * cx_m + 1.0;
            let q_b = 2.0 * (cx_m * cx_b - cx_m * seat_c.1 - seat_c.0);
            let q_c = seat_c.1 * seat_c.1 - r * r + seat_c.0 * seat_c.0 - 2.0 * cx_b * seat_c.1
                + cx_b * cx_b;
            let cx_x = (-q_b - f32::sqrt(q_b * q_b - 4.0 * q_a * q_c)) / (2.0 * q_a);

            // Seating curve/Transitional curve junction
            let x = (cx_x, cx_m * cx_x + cx_b);

            // Calculate line cy, angle B past cx
            let cy_m = -f32::tan(a - b);
            let cy_b = c.1 - cy_m * c.0;

            // Calculate point y (E along cy from c)
            // http://www.physicsforums.com/showthread.php?t=419561
            let y_x = c.0 - e / f32::sqrt(1.0 + cy_m * cy_m);

            // Transitional curve/Tangent line junction
            let y = (y_x, cy_m * y_x + cy_b);

            //  Solve for circle T with radius E which passes through x and y
            let z = ((x.0 + y.0) / 2.0, (x.1 + y.1) / 2.0);
            let x_diff = y.0 - x.0;
            let y_diff = y.1 - x.1;
            let q = f32::sqrt(x_diff * x_diff + y_diff * y_diff);

            println!("e^2: {}", e * e);
            println!("(q/2)^2: {}", (q / 2.0) * (q / 2.0));
            println!("sqrt: {}", f32::sqrt(e * e - (q / 2.0) * (q / 2.0)));

            let t_x = z.0 + f32::sqrt(e * e - (q / 2.0) * (q / 2.0)) * (x.1 - y.1) / q;
            let t_y = z.1 + f32::sqrt(e * e - (q / 2.0) * (q / 2.0)) * (y.0 - x.0) / q;

            // Transitional curve center
            let tran_c = (t_x, t_y);

            println!("tran_c: {:?}", tran_c);

            let tanl_m = -(tran_c.0 - y.0) / (tran_c.1 - y.1);
            let tanl_b = -y.0 * tanl_m + y.1;

            let t_off = (y.0 - 10.0, tanl_m * (y.0 - 10.0) + tanl_b);

            // Topping curve center
            let top_c = (-w, -pr + v);
            println!("TOPPING CURVE CENTER: {:?}", top_c);

            println!("top_c: {:?}, tanl_m: {}, tanl_b: {}", top_c, tanl_m, tanl_b);

            // Adjust F to force topping curve tangent to tangent line
            let f = f32::abs(top_c.1 - tanl_m * top_c.0 - tanl_b)
                / f32::sqrt(tanl_m * tanl_m + 1.0)
                * 1.0001;

            println!("f: {}", f);

            // Find intersection point between topping curve and tangent line
            let tta = tanl_m * tanl_m + 1.0;
            let ttb = 2.0 * (tanl_m * tanl_b - tanl_m * top_c.1 - top_c.0);
            let ttc = top_c.1 * top_c.1 - f * f + top_c.0 * top_c.0 - 2.0 * tanl_b * top_c.1
                + tanl_b * tanl_b;
            let tanl_x = (-ttb - f32::sqrt(ttb * ttb - 4.0 * tta * ttc)) / (2.0 * tta);

            // Tagent line/Topping curve junction
            let tanl = (tanl_x, tanl_m * tanl_x + tanl_b);

            // Calculate tip line, angle t_inc/2 from Y axis
            let tip_m = -f32::tan(PI / 2.0 + t_inc / 2.0);
            let tip_b = 0.0;

            // Calculate intersection of tip line with topping curve
            let ta = tip_m * tip_m + 1.0;
            let tb = 2.0 * (tip_m * tip_b - tip_m * top_c.1 - top_c.0);
            let tc = top_c.1 * top_c.1 - f * f + top_c.0 * top_c.0 - 2.0 * tip_b * top_c.1
                + tip_b * tip_b;

            println!("a{} b{} c{}", ta, tb, tc);

            let tip_x = (-tb - f32::sqrt(tb * tb - 4.0 * ta * tc)) / (2.0 * ta);

            // Topping curve top
            let tip = (tip_x, tip_m * tip_x + tip_b);
            println!("TOPPING CURVE TOP: {:?}", tip);

            // Rotate points by theta
            let rotated_tip = Sprocket::rotate_vector(tip, theta);
            let rotated_top_c = Sprocket::rotate_vector(top_c, theta);
            let rotated_x: (f32, f32) = Sprocket::rotate_vector(x, theta);
            let rotated_y: (f32, f32) = Sprocket::rotate_vector(y, theta);
            let rotated_tanl = Sprocket::rotate_vector(tanl, theta);

            let rotated_inverted_x = Sprocket::rotate_vector(Sprocket::invert_x(x), theta);
            let rotated_inverted_y = Sprocket::rotate_vector(Sprocket::invert_x(y), theta);
            let rotated_inverted_tanl: (f32, f32) =
                Sprocket::rotate_vector(Sprocket::invert_x(tanl), theta);
            let rotated_inverted_tip: (f32, f32) =
                Sprocket::rotate_vector(Sprocket::invert_x(tip), theta);

            if theta == 0.0 {
                points.push(DVec2 {
                    x: tip.0 as f64,
                    y: tip.1 as f64,
                });
            }

            points.push(DVec2 {
                x: rotated_tanl.0 as f64,
                y: rotated_tanl.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_y.0 as f64,
                y: rotated_y.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_x.0 as f64,
                y: rotated_x.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_inverted_x.0 as f64,
                y: rotated_inverted_x.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_inverted_y.0 as f64,
                y: rotated_inverted_y.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_inverted_tanl.0 as f64,
                y: rotated_inverted_tanl.1 as f64,
            });
            points.push(DVec2 {
                x: rotated_inverted_tip.0 as f64,
                y: rotated_inverted_tip.1 as f64,
            });
        }

        points
    }

    fn rotate_vector(vec: (f32, f32), angle_radians: f32) -> (f32, f32) {
        let cos_theta = f32::cos(angle_radians);

        let sin_theta = f32::sin(angle_radians);

        let rotation_matrix = [[cos_theta, -sin_theta], [sin_theta, cos_theta]];

        let new_x = rotation_matrix[0][0] * vec.0 + rotation_matrix[0][1] * vec.1;

        let new_y = rotation_matrix[1][0] * vec.0 + rotation_matrix[1][1] * vec.1;

        (new_x, new_y)
    }
}
