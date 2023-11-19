export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'main' : IDL.Func([IDL.Text, IDL.Text], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
