use eco_fairness_guard::GuardError as EcoGuardError;

// inside TsafeCortexGate::authorizerequest

// 3. Neurorights guard.
if let Err(reason) = self.guardians.neurorights_guard.check(&req.action) {
    self.donutlogger.log_reject(&req, &reason.code());
    return AuthorizationResult::Rejected(RejectionReason {
        code: reason.code(),
        message: reason.to_string(),
    });
}

// 4. RoH guard.
if let Err(reason) = self.guardians.roh_guard.check(&req.action) {
    self.donutlogger.log_reject(&req, &reason.code());
    return AuthorizationResult::Rejected(RejectionReason {
        code: reason.code(),
        message: reason.to_string(),
    });
}

// 5. Eco + fairness guard (GraceEquityKernel).
if let Err(e) = self.guardians.eco_guard.check(&req.action, &req.route) {
    tracing::warn!(
        "EcoFairnessGuard rejected route {} for {}: {e}",
        req.route.as_str(),
        req.subjectid
    );
    self.donutlogger.log_reject(&req, "ECO_FAIRNESS");
    return AuthorizationResult::Rejected(RejectionReason {
        code: "ECO_FAIRNESS".into(),
        message: e.to_string(),
    });
}

// 6. EVOLVE token verifier, etc.
