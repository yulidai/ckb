use crate::{core, packed, prelude::*};

/*
 * Blockchain
 */

impl<'r> packed::ScriptHashTypeReader<'r> {
    pub fn check_data(&self) -> bool {
        core::ScriptHashType::verify_value(self.as_slice()[0])
    }
}

impl<'r> packed::DepTypeReader<'r> {
    pub fn check_data(&self) -> bool {
        core::DepType::verify_value(self.as_slice()[0])
    }
}

impl<'r> packed::ScriptReader<'r> {
    pub fn check_data(&self) -> bool {
        self.hash_type().check_data()
    }
}

impl<'r> packed::ScriptOptReader<'r> {
    pub fn check_data(&self) -> bool {
        self.to_opt()
            .map(|i| i.hash_type().check_data())
            .unwrap_or(true)
    }
}

impl<'r> packed::CellOutputReader<'r> {
    pub fn check_data(&self) -> bool {
        self.lock().check_data() && self.type_().check_data()
    }
}

impl<'r> packed::CellOutputVecReader<'r> {
    pub fn check_data(&self) -> bool {
        self.iter().all(|i| i.check_data())
    }
}

impl<'r> packed::CellDepReader<'r> {
    pub fn check_data(&self) -> bool {
        self.dep_type().check_data()
    }
}

impl<'r> packed::CellDepVecReader<'r> {
    pub fn check_data(&self) -> bool {
        self.iter().all(|i| i.check_data())
    }
}

impl<'r> packed::RawTransactionReader<'r> {
    pub fn check_data(&self) -> bool {
        self.cell_deps().check_data() && self.outputs().check_data()
    }
}

impl<'r> packed::TransactionReader<'r> {
    pub fn check_data(&self) -> bool {
        self.raw().check_data()
    }
}

impl<'r> packed::TransactionVecReader<'r> {
    pub fn check_data(&self) -> bool {
        self.iter().all(|i| i.check_data())
    }
}

impl<'r> packed::BlockReader<'r> {
    pub fn check_data(&self) -> bool {
        self.transactions().check_data()
    }
}

/*
 * Network
 */

impl<'r> packed::BlockTransactionsReader<'r> {
    pub fn check_data(&self) -> bool {
        self.transactions().check_data()
    }
}

impl<'r> packed::RelayTransactionReader<'r> {
    pub fn check_data(&self) -> bool {
        self.transaction().check_data()
    }
}

impl<'r> packed::RelayTransactionVecReader<'r> {
    pub fn check_data(&self) -> bool {
        self.iter().all(|i| i.check_data())
    }
}

impl<'r> packed::RelayTransactionsReader<'r> {
    pub fn check_data(&self) -> bool {
        self.transactions().check_data()
    }
}

impl<'r> packed::SendBlockReader<'r> {
    pub fn check_data(&self) -> bool {
        self.block().check_data()
    }
}

#[cfg(test)]
mod tests {
    use crate::{packed, prelude::*};

    fn create_transaction(
        outputs: &[&packed::CellOutput],
        cell_deps: &[&packed::CellDep],
    ) -> packed::Transaction {
        let outputs = outputs
            .iter()
            .map(|d| d.to_owned().to_owned())
            .collect::<Vec<packed::CellOutput>>();
        let cell_deps = cell_deps
            .iter()
            .map(|d| d.to_owned().to_owned())
            .collect::<Vec<packed::CellDep>>();
        let raw = packed::RawTransaction::new_builder()
            .outputs(outputs.into_iter().pack())
            .cell_deps(cell_deps.into_iter().pack())
            .build();
        packed::Transaction::new_builder().raw(raw).build()
    }

    fn test_check_data_via_transaction(
        expected: bool,
        outputs: &[&packed::CellOutput],
        cell_deps: &[&packed::CellDep],
    ) {
        let tx = create_transaction(outputs, cell_deps);
        assert_eq!(tx.as_reader().check_data(), expected);
    }

    #[test]
    fn check_data() {
        let ht_right = packed::ScriptHashType::new_builder().set([1]).build();
        let ht_error = packed::ScriptHashType::new_builder().set([2]).build();
        let dt_right = packed::DepType::new_builder().set([1]).build();
        let dt_error = packed::DepType::new_builder().set([2]).build();

        let script_right = packed::Script::new_builder()
            .hash_type(ht_right.clone())
            .build();
        let script_error = packed::Script::new_builder()
            .hash_type(ht_error.clone())
            .build();

        let script_opt_right = packed::ScriptOpt::new_builder()
            .set(Some(script_right.clone()))
            .build();
        let script_opt_error = packed::ScriptOpt::new_builder()
            .set(Some(script_error.clone()))
            .build();

        let output_right1 = packed::CellOutput::new_builder()
            .lock(script_right.clone())
            .build();
        let output_right2 = packed::CellOutput::new_builder()
            .type_(script_opt_right.clone())
            .build();
        let output_error1 = packed::CellOutput::new_builder()
            .lock(script_error.clone())
            .build();
        let output_error2 = packed::CellOutput::new_builder()
            .type_(script_opt_error.clone())
            .build();
        let output_error3 = packed::CellOutput::new_builder()
            .lock(script_right.clone())
            .type_(script_opt_error.clone())
            .build();
        let output_error4 = packed::CellOutput::new_builder()
            .lock(script_error.clone())
            .type_(script_opt_right.clone())
            .build();

        let cell_dep_right = packed::CellDep::new_builder()
            .dep_type(dt_right.clone())
            .build();
        let cell_dep_error = packed::CellDep::new_builder()
            .dep_type(dt_error.clone())
            .build();

        test_check_data_via_transaction(true, &[], &[]);
        test_check_data_via_transaction(true, &[&output_right1], &[&cell_dep_right]);
        test_check_data_via_transaction(
            true,
            &[&output_right1, &output_right2],
            &[&cell_dep_right, &cell_dep_right],
        );
        test_check_data_via_transaction(false, &[&output_error1], &[]);
        test_check_data_via_transaction(false, &[&output_error2], &[]);
        test_check_data_via_transaction(false, &[&output_error3], &[]);
        test_check_data_via_transaction(false, &[&output_error4], &[]);
        test_check_data_via_transaction(false, &[], &[&cell_dep_error]);
        test_check_data_via_transaction(
            false,
            &[
                &output_right1,
                &output_right2,
                &output_error1,
                &output_error2,
                &output_error3,
                &output_error4,
            ],
            &[&cell_dep_right, &cell_dep_error],
        );
    }
}
