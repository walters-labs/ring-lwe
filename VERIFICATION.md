# Verification for ring-lwe 0.1.9

This release is prepared against the fixed-parameter Cryptol/SAW checkpoint
in https://github.com/walters-labs/ntt/tree/v0.1.10/verification .

The transform tuples are (1024, 12289, 10302), (2048, 12289, 1945), and
(2048, 151019521, 51689479), with arbitrary signed i64 coefficients.
The consumer entry proofs cover real polynomial construction, padding,
NTT multiplication, folding/centering, truncation, and trimming metadata.
They cover the three fixed array construction paths, including every logical
polynomial value in the bounded domains. Direct calls on arbitrary independent
heap layouts and whole-cryptosystem security are outside those proofs.

Proof reuse has a separate trusted checker for exact contracts, parameter
instances, prior successful proofs, and reachable MIR graph identity. It uses
SAW's imported MIR contracts; a standalone SAW exit does not establish the
composed result. See verification/consumer-call/README.md in the NTT repository.

The release integration check packages all three new crate versions, checks
their normalized dependency requirements, tests the extracted packages, and
compares their rebuilt consumer entry MIR with the accepted proof artifact.
Before publication, the two unpublished dependency versions are supplied from
the exact extracted crate archives. Publish ntt 0.1.10 first, ring-lwe 0.1.9
second, and module-lwe 0.1.6 last, rerunning registry-backed package verification
as each prerequisite becomes available.
