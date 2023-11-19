import { Canister, query, text, update, Void, nat, Record, Vec, bool, ic, StableBTreeMap, Principal, Opt } from 'azle';

import { VK } from './constants';
import groth16Verify from './zk/groth16_verify';
import { verify } from "@zk-kit/groth16"


const Groth16Record = Record({
    curve: text,
    pi_a: Vec(text),
    pi_b: Vec(Vec(text)),
    pi_c: Vec(text),
    protocol: text

})

const Proof = Record({
    modulus: text,
    pcd: Groth16Record
})

const Emails = Record({
    emails: Vec(text)
})

const userAadharMap = StableBTreeMap(Principal, Proof, 0)
const userEmailMap = StableBTreeMap(Principal, Emails, 0)


export function splitToWords(
    number: bigint,
    wordsize: bigint,
    numberElement: bigint
) {
    let t = number
    const words: string[] = []
    for (let i = BigInt(0); i < numberElement; ++i) {
        const baseTwo = BigInt(2)

        words.push(`${t % BigInt(Math.pow(Number(baseTwo), Number(wordsize)))}`)
        t = BigInt(t / BigInt(Math.pow(Number(BigInt(2)), Number(wordsize))))
    }
    if (!(t == BigInt(0))) {
        throw `Number ${number} does not fit in ${(
            wordsize * numberElement
        ).toString()} bits`
    }
    return words
}
// This is a global variable that is stored on the heap
let message = '';
export default Canister({
    // Query calls complete quickly because they do not go through consensus
    getMessage: query([], text, () => {
        return message;
    }),
    // Update calls take a few seconds to complete
    // This is because they persist state changes and go through consensus
    setMessage: update([text], Void, (newMessage) => {
        message = newMessage; // This change will be persisted
    }),

    verifyAadhar: update([text, Groth16Record], bool, (modulus, groth16Proof) => {
        ic.print(modulus, "modules")
        return verify(
            VK,
            {
                publicSignals: [...splitToWords(BigInt(modulus), BigInt(64), BigInt(32))],
                proof: {
                    curve: groth16Proof.curve,
                    pi_a: groth16Proof.pi_a,
                    pi_b: groth16Proof.pi_b,
                    pi_c: groth16Proof.pi_c,
                    protocol: groth16Proof.protocol
                }
            }

        )
    }),

    linkAadharToPCD: update([text, Proof], bool, (principle, proof) => {
        userAadharMap.insert(Principal.fromText(principle), proof)
        return true
    }),

    getAadharProof: query([text], Opt(Proof), (principleText) => {
        const a = userAadharMap.get(Principal.fromText(principleText));
        ic.print(a, "aaa")
        return a;
    })


});

