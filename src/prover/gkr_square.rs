// an implementation of the GKR^2 protocol
//! This module implements the core GKR^2 IOP.

use ark_std::{end_timer, start_timer};
use transcript::{Transcript, TranscriptInstance};

use crate::{sumcheck_prove_gkr_square_layer, Circuit, GKRConfig, GkrScratchpad, MultiLinearPoly};

pub fn gkr_square_prove<C: GKRConfig>(
    circuit: &Circuit<C>,
    sp: &mut GkrScratchpad<C>,
    transcript: &mut TranscriptInstance<C::FiatShamirHashType>,
) -> (C::Field, Vec<C::ChallengeField>) {
    let timer = start_timer!(|| "gkr^2 prove");
    let layer_num = circuit.layers.len();

    let mut rz0 = vec![];
    for _i in 0..circuit.layers.last().unwrap().output_var_num {
        rz0.push(transcript.generate_challenge::<C::ChallengeField>());
    }

    let circuit_output = &circuit.layers.last().unwrap().output_vals;
    let claimed_v = MultiLinearPoly::eval_circuit_vals_at_challenge::<C>(
        circuit_output,
        &rz0,
        &mut sp.hg_evals,
    );

    for i in (0..layer_num).rev() {
        rz0 = sumcheck_prove_gkr_square_layer(&circuit.layers[i], &rz0, transcript, sp);

        log::trace!("Layer {} proved", i);
        log::trace!("rz0.0: {:?}", rz0[0]);
        log::trace!("rz0.1: {:?}", rz0[1]);
        log::trace!("rz0.2: {:?}", rz0[2]);
    }

    end_timer!(timer);
    (claimed_v, rz0)
}
