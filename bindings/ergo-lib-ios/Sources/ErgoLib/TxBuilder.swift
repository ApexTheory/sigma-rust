import Foundation
import ErgoLibC

/// ``UnsignedTransaction`` builder
class TxBuilder {
    internal var pointer: TxBuilderPtr
    
    /// Creates new ``TxBuilder``
    /// - Parameters
    ///  - `boxSelection`: selected input boxes
    ///  - `outputCandidates` - output boxes to be "created" in this transaction,
    ///  - `currentHeight`: chain height that will be used in additionally created boxes (change, miner's fee, etc.),
    ///  - `feeAmount`: miner's fee,
    ///  - `changeAddress`: change (inputs - outputs) will be sent to this address,
    ///  - `minChangeValue`: minimal value of the change to be sent to `change_address`, value less than that
    ///     will be given to miners,
    init(
        boxSelection : BoxSelection,
        outputCandidates: ErgoBoxCandidates,
        currentHeight: UInt32,
        feeAmount: BoxValue,
        changeAddress: Address,
        minChangeValue: BoxValue
    ) {
        var ptr: TxBuilderPtr?
        ergo_lib_tx_builder_new(
            boxSelection.pointer,
            outputCandidates.pointer,
            currentHeight,
            feeAmount.pointer,
            changeAddress.pointer,
            minChangeValue.pointer,
            &ptr
        )
        self.pointer = ptr!
    }
    
    /// Suggested transaction fee (semi-default value used across wallets and dApps as of Oct 2020)
    static func SUGGESTED_TX_FEE() -> BoxValue {
        var ptr: BoxValuePtr?
        ergo_lib_tx_builder_suggested_tx_fee(&ptr)
        return BoxValue(withRawPointer: ptr!)
    }
    
    /// Set transaction's ``DataInputs``
    func setDataInputs(dataInputs: DataInputs) {
        ergo_lib_tx_builder_set_data_inputs(self.pointer, dataInputs.pointer)
    }
    
    /// Get ``DataInputs``
    func getDataInputs() -> DataInputs {
        var ptr: DataInputsPtr?
        ergo_lib_tx_builder_data_inputs(self.pointer, &ptr)
        return DataInputs(withRawPointer: ptr!)
    }
    
    /// Build the ``UnsignedTransaction``
    func build() throws -> UnsignedTransaction {
        var ptr: UnsignedTransactionPtr?
        let error = ergo_lib_tx_builder_build(self.pointer, &ptr)
        try checkError(error)
        return UnsignedTransaction(withRawPointer: ptr!)
    }
    
    /// Get ``BoxSelection``
    func getBoxSelection() -> BoxSelection {
        var ptr: BoxSelectionPtr?
        ergo_lib_tx_builder_box_selection(self.pointer, &ptr)
        return BoxSelection(withRawPointer: ptr!)
    }
    
    /// Get ``OutputCandidates``
    func getOutputCandidates() -> ErgoBoxCandidates {
        var ptr: ErgoBoxCandidatesPtr?
        ergo_lib_tx_builder_output_candidates(self.pointer, &ptr)
        return ErgoBoxCandidates(withRawPointer: ptr!)
    }
    
    /// Get current height
    func getCurrentHeight() -> UInt32 {
        return ergo_lib_tx_builder_current_height(self.pointer)
    }
    
    /// Get fee amount
    func getFeeAmount() -> BoxValue {
        var ptr: BoxValuePtr?
        ergo_lib_tx_builder_fee_amount(self.pointer, &ptr)
        return BoxValue(withRawPointer: ptr!)
    }
    
    /// Get change ``Address``
    func getChangeAddress() -> Address {
        var ptr: AddressPtr?
        ergo_lib_tx_builder_change_address(self.pointer, &ptr)
        return Address(withRawPointer: ptr!)
    }
    
    /// Get min change value
    func getMinChangeValue() -> BoxValue {
        var ptr: BoxValuePtr?
        ergo_lib_tx_builder_min_change_value(self.pointer, &ptr)
        return BoxValue(withRawPointer: ptr!)
    }
    
    deinit {
        ergo_lib_tx_builder_delete(self.pointer)
    }
}
