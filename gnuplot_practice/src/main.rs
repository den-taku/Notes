use gnuplot::*;

fn f(x: f64) -> f64 {
    x * x - 2.0
}

fn df(x: f64) -> f64 {
    2.0 * x
}

fn main() {
    let mut t = vec![];
    for i in 0..100 {
        t.push(i as f64 * 0.1 - 5.0);
    }
    let mut fg = Figure::new();
    {
        let axec = fg.axes2d().set_y_log(Some(10.0));
        axec.set_x_axis(true, &[]);
        axec.set_x_range(Fix(0.0), Fix(5.0));
        axec.set_y_range(Fix(-5.0), Fix(30.0));
        axec.lines(t.iter(), t.iter().map(|&x| f(x)), &[Color("red")]);

        let mut x = 4.0;
        while f(x).abs() > 1e-10 {
            axec.points(&[x], &[f(x)], &[Color("blue"), PointSymbol('O')]);
            axec.lines(&[x, x], &[-5.0, 30.0], &[Color("blue")]);
            axec.lines(
                t.iter(),
                t.iter().map(|&p| df(x) * (p - x) + f(x)),
                &[Color("black")],
            );
            x = x - f(x) / df(x);
        }
        println!("solution: {}", x);
    }
    let _ = fg.show();
}
