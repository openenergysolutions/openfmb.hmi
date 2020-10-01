export const getModules = () => {
  return Object.keys(OPENFMB_MODELS);
};

export const getProfilesByModule = (name: string) => {
  return OPENFMB_MODELS[name] ? OPENFMB_MODELS[name] : []
};
