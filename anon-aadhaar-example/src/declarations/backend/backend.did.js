export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'getAadharProof' : IDL.Func(
        [IDL.Text],
        [
          IDL.Opt(
            IDL.Record({
              'pcd' : IDL.Record({
                'protocol' : IDL.Text,
                'pi_a' : IDL.Vec(IDL.Text),
                'pi_b' : IDL.Vec(IDL.Vec(IDL.Text)),
                'pi_c' : IDL.Vec(IDL.Text),
                'curve' : IDL.Text,
              }),
              'modulus' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
    'getMessage' : IDL.Func([], [IDL.Text], ['query']),
    'linkAadharToPCD' : IDL.Func(
        [
          IDL.Text,
          IDL.Record({
            'pcd' : IDL.Record({
              'protocol' : IDL.Text,
              'pi_a' : IDL.Vec(IDL.Text),
              'pi_b' : IDL.Vec(IDL.Vec(IDL.Text)),
              'pi_c' : IDL.Vec(IDL.Text),
              'curve' : IDL.Text,
            }),
            'modulus' : IDL.Text,
          }),
        ],
        [IDL.Bool],
        [],
      ),
    'setMessage' : IDL.Func([IDL.Text], [], []),
    'verifyAadhar' : IDL.Func(
        [
          IDL.Text,
          IDL.Record({
            'protocol' : IDL.Text,
            'pi_a' : IDL.Vec(IDL.Text),
            'pi_b' : IDL.Vec(IDL.Vec(IDL.Text)),
            'pi_c' : IDL.Vec(IDL.Text),
            'curve' : IDL.Text,
          }),
        ],
        [IDL.Bool],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
