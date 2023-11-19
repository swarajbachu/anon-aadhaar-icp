import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'getAadharProof' : ActorMethod<
    [string],
    [] | [
      {
        'pcd' : {
          'protocol' : string,
          'pi_a' : Array<string>,
          'pi_b' : Array<Array<string>>,
          'pi_c' : Array<string>,
          'curve' : string,
        },
        'modulus' : string,
      }
    ]
  >,
  'getMessage' : ActorMethod<[], string>,
  'linkAadharToPCD' : ActorMethod<
    [
      string,
      {
        'pcd' : {
          'protocol' : string,
          'pi_a' : Array<string>,
          'pi_b' : Array<Array<string>>,
          'pi_c' : Array<string>,
          'curve' : string,
        },
        'modulus' : string,
      },
    ],
    boolean
  >,
  'setMessage' : ActorMethod<[string], undefined>,
  'verifyAadhar' : ActorMethod<
    [
      string,
      {
        'protocol' : string,
        'pi_a' : Array<string>,
        'pi_b' : Array<Array<string>>,
        'pi_c' : Array<string>,
        'curve' : string,
      },
    ],
    boolean
  >,
}
