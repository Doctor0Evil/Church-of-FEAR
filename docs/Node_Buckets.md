Node_Buckets are the way Auto_Church groups XR‑grid nodes into a few governed capacity tiers so fairness and Tsafe policies can be applied per “bucket” instead of per machine. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/9db70f08-688a-47d5-8aa9-b8f1809b2ff2/please-create-docs-infranet-md-3QCUVaPJQpSb8WxT3w5CIA.md)

### Core idea

- A Node_Bucket is a named class of nodes (for example: home‑hosted NeuroPC, community lab, city‑scale cluster) that share similar power budgets, RoH envelopes, and Tsafe viability kernels. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/9db70f08-688a-47d5-8aa9-b8f1809b2ff2/please-create-docs-infranet-md-3QCUVaPJQpSb8WxT3w5CIA.md)
- Each bucket has its own eco‑envelope, RoH weights, and fairness parameters (GraceEquityKernel, EquityClasses), so schedulers can reason in terms of “this KO can run on a `community_bucket` but not on a `home_bucket`.” [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/9db70f08-688a-47d5-8aa9-b8f1809b2ff2/please-create-docs-infranet-md-3QCUVaPJQpSb8WxT3w5CIA.md)

### Why they matter for Auto_Church

- They let the EcoFairnessGuard and GraceEquityKernel apply different ceilings and equity floors depending on where an Auto_Church task is going to run, without hard‑coding per‑device rules. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/9db70f08-688a-47d5-8aa9-b8f1809b2ff2/please-create-docs-infranet-md-3QCUVaPJQpSb8WxT3w5CIA.md)
- They give the governance layer a clean handle for policy: EVOLVE can tighten or relax envelopes bucket‑by‑bucket (for example, “raise eco repair capacity on `repair_bucket` nodes”) instead of editing dozens of raw node configs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/2a657fd1-afb9-4137-93a7-a26d80e9f642/architecting-verifiable-safety-rae5X4JZRh6OB7BgEPQgQA.md)

### How they’re used

- The Tsafe scheduler tags each node with a `Node_Bucket` label and only considers buckets whose RoH, eco and equity envelopes admit the proposed Knowledge Object (KO). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/2a657fd1-afb9-4137-93a7-a26d80e9f642/architecting-verifiable-safety-rae5X4JZRh6OB7BgEPQgQA.md)
- In audit/sim labs, CI can spin up synthetic nodes per bucket type and prove that under load, no bucket can violate its RoH 0.3 ceiling, neurorights, or eco‑fairness constraints. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_90c8124f-19ff-4978-9a68-968f40874efe/2a657fd1-afb9-4137-93a7-a26d80e9f642/architecting-verifiable-safety-rae5X4JZRh6OB7BgEPQgQA.md)
