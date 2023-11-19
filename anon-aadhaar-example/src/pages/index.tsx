/* eslint-disable react/no-unescaped-entities */
import {
  LogInWithAnonAadhaar,
  LogInWithAnonAadhaarV2,
  useAnonAadhaar,

} from "anon-aadhaar-react";
import { Dispatch, useEffect, useState, SetStateAction } from "react";
import { Stepper } from "../components/Stepper";
import { Toggle } from "../components/Toggles";
import { ProofContainer } from "@/components/ProofContainer";
import { useRouter } from "next/router";
import { UserStatus } from "@/interface";
import { TestFiles } from "@/components/TestFiles";
import { AuthClient } from "@dfinity/auth-client";
import { Button, Text } from "@chakra-ui/react";
import { AnonAadhaarPCD, VK_URL, WASM_URL, ZKEY_URL, init, verify } from "anon-aadhaar-pcd";
import { HttpAgent } from "@dfinity/agent";
import { createActor } from "@/declarations/backend";
import { createActor as createVerifier } from "@/declarations/rust_backend_backend"
// import { createActor } from "../declarations/backend/index"
// import { HttpAgent } from "@dfinity/agent";

type HomeProps = {
  setUserStatus: Dispatch<SetStateAction<UserStatus>>;
};
function splitToWords(
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
export default function Home({ setUserStatus }: HomeProps) {
  // Use the Country Identity hook to get the status of the user.
  const [anonAadhaar] = useAnonAadhaar();
  const [withCert, setWithCert] = useState<boolean>(false);
  const router = useRouter();
  const [authClient, setAuthClient] = useState<AuthClient>()
  const [isAuthenticated, setAuthenticated] = useState(false)
  useEffect(() => {
    if (anonAadhaar.status === "logged-in") setPcd(anonAadhaar.pcd);
  }, [anonAadhaar]);





  const [voted, setVoted] = useState(false);
  const [pcd, setPcd] = useState<AnonAadhaarPCD>();







  const [isLoading, setLoading] = useState(false)




  const verifyOnClick = async () => {
    setLoading(true)
    await init({
      isWebEnv: true,
      vkeyURL: VK_URL,
      wasmURL: WASM_URL,
      zkeyURL: ZKEY_URL
    })
    if (pcd) {
      const input = [
        pcd.proof.nullifier.toString(),
        ...splitToWords(BigInt(pcd.proof.modulus), BigInt(64), BigInt(32)),
        pcd.proof.app_id.toString(),
      ]

      console.log({ input })
      console.log(JSON.stringify(input), "inpu")
      // const valid = await verify(
      //   pcd
      // )
      const verifier = createVerifier("icwh7-aaaaa-aaaap-abroq-cai", {
        agent: new HttpAgent({ host: "https://ic0.app" })

      })
      const valid: boolean = await verifier.main(
        JSON.stringify({
          pi_a: pcd.proof.proof.pi_a,
          pi_b: pcd.proof.proof.pi_b,
          pi_c: pcd.proof.proof.pi_c,
        }),
        JSON.stringify(input)
      )

      console.log({ valid }, "valid")
      const identity = authClient?.getIdentity()
      console.log(identity?.getPrincipal().toString(), "iiiiii")
      const actor = await createActor("ilvmd-wiaaa-aaaap-abrpa-cai", {
        agent: new HttpAgent({ identity, host: "https://ic0.app" })
      })

      console.log(pcd, "id calimS")
      console.log(JSON.stringify(pcd.proof.proof), "hiljkjl")
      const done = await actor.linkAadharToPCD(
        identity?.getPrincipal().toString() ?? "a",
        {
          modulus: pcd.proof.modulus.toString(),
          pcd: {
            curve: pcd.proof.proof.curve,
            pi_a: pcd.proof.proof.pi_a,
            pi_b: pcd.proof.proof.pi_b,
            pi_c: pcd.proof.proof.pi_c,
            protocol: pcd.proof.proof.protocol
          }
        }
      )
      if (valid) {
        alert("Your Aadhar has been verified and Linked")
      }
      console.log({ done }, "doneee")
    }
    setLoading(false)


  }



  useEffect(() => {
    if (anonAadhaar.status === "logged-in") setPcd(anonAadhaar.pcd);
  }, [anonAadhaar]);

  const connectToII = () => {

    if (!authClient) {
      AuthClient.create().then(resolve => {
        resolve.login({
          identityProvider: "https://identity.ic0.app"

        }).then(() => {
          console.log(resolve, "resolve")
          setAuthenticated(true)
          setAuthClient(resolve)
        })
      })

    }
  }

  return (
    <>
      <main className="flex flex-col min-h-[75vh] mx-auto rounded-2xl w-full sm:max-w-screen-sm p-4 sm:p-8 justify-between">
        <h1 className="font-bold text-sm sm:text-2xl">
          KYC II
        </h1>
        <div className="text-sm sm:text-lg">
          First Login Using Your II and then Link Your Aadhaar Card with it
        </div>

        {/* Import the Connect Button component */}
        <div className="flex w-full place-content-center">
          {withCert ? <LogInWithAnonAadhaar /> : <LogInWithAnonAadhaarV2 />}
        </div>

        {
          anonAadhaar.status == "logged-in" && <Text>Proof Generated with Modulus {anonAadhaar.pcd.proof.modulus.toString()}</Text>
        }

        {!isAuthenticated ? <Button bg={"rebeccapurple"} onClick={connectToII}>Connect To II</Button> : <Button onClick={async () => {
          console.log("skdfldjfljskfl")
          console.log({ authClient })
          await authClient?.logout()
          window.location.reload()
        }}>Logout</Button>}



        {anonAadhaar.status == 'logged-in' && anonAadhaar.pcd ? <Button isLoading={isLoading} size="xl" padding="16px" bg={"green"} onClick={verifyOnClick}>Verify...</Button> : null}





      </main>
    </>
  );
}
