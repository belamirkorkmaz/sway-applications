import { Button, Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import { InputFieldComponent, InputNumberComponent } from "../../common/components";
import { validateAddress, validateContractId, validateData } from "../../common/utils";
import { IdentityInput } from "../../../contracts/MultisigContractAbi";

interface ComponentInput {
    recipient: string
}

export function ExecuteHashComponent( { recipient }: ComponentInput ) {
    const [address, setAddress] = useState("")
    const [assetAmount, setAssetAmount] = useState(0)
    const [nonce, setNonce] = useState(0)
    const [data, setData] = useState("")
    
    const { contract, isLoading, isError } = useContract()
    const [isConnected] = useIsConnected();

    async function getHash() {
        let identity: IdentityInput;

        if (recipient === "address") {
            let { address: user, isError } = validateAddress(address);
            if (isError) return;

            identity = { Address: { value: user } };
        } else {
            let { address: user, isError } = validateContractId(address);
            if (isError) return;

            identity = { ContractId: { value: user } };
        }

        const { data: validatedData, isError } = validateData(data);
        if (isError) return;

        const { value } = await contract!.functions.transaction_hash(validatedData, nonce, identity, assetAmount).get().then(
            null,
            (error) => {
                if (error.logs.length === 0) {
                    toast.error("Unknown error occurred during contract call.", { duration: 10000 });
                } else {
                    toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, { duration: 10000 });
                }
                return;
            }
        );

        toast.success(`Hash: ${value}`, { duration: 10000 });
    }

    return (
        <>
            <Stack>
                <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}>
                    Hash for execution
                </Heading>

                <InputFieldComponent onChange={setAddress} text="Recipient address" placeholder="0x80d5e8c2be..." />
                <InputNumberComponent onChange={setAssetAmount} text="Asset amount" placeholder="1.0" />
                <InputNumberComponent onChange={setNonce} text="Nonce" placeholder="3" />
                <InputFieldComponent onChange={setData} text="Data to sign" placeholder="0x252afeeb6e..." />

                <Button
                    color="accent"
                    onPress={getHash}
                    size="lg"
                    variant="solid"
                    isDisabled={!isConnected}
                    css={{ marginTop: "$1", boxShadow: "0px 0px 1px 1px" }}
                >
                    Create hash
                </Button>
            </Stack>
        </>
    );
}