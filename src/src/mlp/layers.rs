use crate::src::mlp::tansig::{sigmoid_approx, tansig_approx};
use ndarray::{
    aview1, aview_mut1, azip, ArrayView1, ArrayView2, ArrayViewMut1, Axis, ShapeBuilder as _,
};

pub const WEIGHTS_SCALE: f32 = 1.0f32 / 128f32;

fn gemm_accum(mut out: ArrayViewMut1<f32>, weights: ArrayView2<i8>, x: ArrayView1<f32>) {
    azip!((out in &mut out, weights in weights.rows()) {
        azip!((&w in weights, &x in x) {
            let v = (w as f32) * x;
            *out += v;
        })
    });
}

pub enum ActivationFunction {
    Tansig,
    Sigmoid,
}

pub struct DenseLayer {
    pub bias: &'static [i8],
    pub input_weights: &'static [i8],
    pub activation: ActivationFunction,
}

impl DenseLayer {
    pub fn nb_inputs(&self) -> usize {
        self.input_weights.len() / self.nb_neurons()
    }

    pub fn nb_neurons(&self) -> usize {
        self.bias.len()
    }

    // TODO: it would be nice to have to do this only once (converting slices to arrays requires some bound checks)
    pub fn as_arrays(&self) -> (ArrayView1<i8>, ArrayView2<i8>) {
        let n = self.nb_neurons();
        let m = self.nb_inputs();
        let col_stride = n;

        let bias = aview1(self.bias);
        let input_weights =
            ArrayView2::from_shape((n, m).strides((1, col_stride)), self.input_weights).unwrap();

        (bias, input_weights)
    }

    #[inline]
    pub fn compute(&self, out: &mut [f32], input: &[f32]) {
        assert_eq!(self.nb_neurons(), out.len());
        assert_eq!(self.nb_inputs(), input.len());

        let mut out = aview_mut1(out);
        let input = aview1(input);

        let (bias, input_weights) = self.as_arrays();

        azip!((out in &mut out, &bias in &bias) {
            *out = bias as f32;
        });

        gemm_accum(out.view_mut(), input_weights, input);

        out.mapv_inplace(|out| out * WEIGHTS_SCALE);

        match self.activation {
            ActivationFunction::Tansig => {
                out.mapv_inplace(tansig_approx);
            }
            ActivationFunction::Sigmoid => {
                out.mapv_inplace(sigmoid_approx);
            }
        }
    }
}

pub struct GRULayer {
    // `bias`, `input_weights` and `recurrent_weights` are three concatenated matrices for update, reset and output components
    pub bias: &'static [i8],
    pub input_weights: &'static [i8],
    pub recurrent_weights: &'static [i8],
}

impl GRULayer {
    pub fn nb_inputs(&self) -> usize {
        self.input_weights.len() / self.nb_neurons() / 3
    }

    pub fn nb_neurons(&self) -> usize {
        self.bias.len() / 3
    }

    fn as_arrays(
        &self,
    ) -> (
        [ArrayView1<i8>; 3],
        [ArrayView2<i8>; 3],
        [ArrayView2<i8>; 3],
    ) {
        let m = self.nb_inputs();
        let n = self.nb_neurons();
        let col_stride = 3 * n;

        let bias = aview1(self.bias);
        let input_weights =
            ArrayView2::from_shape((n * 3, m).strides((1, col_stride)), self.input_weights)
                .unwrap();
        let recurrent_weights =
            ArrayView2::from_shape((n * 3, n).strides((1, col_stride)), self.recurrent_weights)
                .unwrap();

        let (bias1, tail) = bias.split_at(Axis(0), n);
        let (bias2, bias3) = tail.split_at(Axis(0), n);

        let (input_weights1, tail) = input_weights.split_at(Axis(0), n);
        let (input_weights2, input_weights3) = tail.split_at(Axis(0), n);

        let (recurrent_weights1, tail) = recurrent_weights.split_at(Axis(0), n);
        let (recurrent_weights2, recurrent_weights3) = tail.split_at(Axis(0), n);

        (
            [bias1, bias2, bias3],
            [input_weights1, input_weights2, input_weights3],
            [recurrent_weights1, recurrent_weights2, recurrent_weights3],
        )
    }

    #[inline]
    pub fn compute(&self, state: &mut [f32], input: &[f32]) {
        const MAX_NEURONS: usize = 32;

        assert_eq!(self.nb_neurons(), state.len());
        assert_eq!(self.nb_inputs(), input.len());

        assert!(self.nb_neurons() < MAX_NEURONS);
        let n = self.nb_neurons();

        let mut state = aview_mut1(state);
        let input = aview1(input);

        let mut tmp: [f32; MAX_NEURONS] = [0.; MAX_NEURONS];
        let mut z: [f32; MAX_NEURONS] = [0.; MAX_NEURONS];
        let mut r: [f32; MAX_NEURONS] = [0.; MAX_NEURONS];
        let mut h: [f32; MAX_NEURONS] = [0.; MAX_NEURONS];

        let mut tmp = aview_mut1(&mut tmp[..n]);
        let mut z = aview_mut1(&mut z[..n]);
        let mut r = aview_mut1(&mut r[..n]);
        let mut h = aview_mut1(&mut h[..n]);

        let (bias, input_weights, recurrent_weights) = self.as_arrays();

        /* Compute update gate. */
        azip!((z in &mut z, &bias in &bias[0]) {
            *z = bias as f32;
        });
        gemm_accum(z.view_mut(), input_weights[0], input);
        gemm_accum(z.view_mut(), recurrent_weights[0], state.view());
        z.mapv_inplace(|z| sigmoid_approx(WEIGHTS_SCALE * z));

        /* Compute reset gate. */
        azip!((r in &mut r, &bias in &bias[1]) {
            *r = bias as f32;
        });
        gemm_accum(r.view_mut(), input_weights[1], input);
        gemm_accum(r.view_mut(), recurrent_weights[1], state.view());
        r.mapv_inplace(|r| sigmoid_approx(WEIGHTS_SCALE * r));

        /* Compute output. */
        azip!((h in &mut h, &bias in &bias[2]) {
            *h = bias as f32;
        });
        azip!((tmp in &mut tmp, &state in &state, &r in &r) {
            *tmp = state * r;
        });
        gemm_accum(h.view_mut(), input_weights[2], input);
        gemm_accum(h.view_mut(), recurrent_weights[2], tmp.view());
        azip!((h in &mut h, &state in &state, &z in &z) {
            *h = z * state + (1.0 - z) * tansig_approx(WEIGHTS_SCALE * *h);
        });
        azip!((state in &mut state, &h in &h) {
            *state = h;
        });
    }
}
