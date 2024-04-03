import fs from "fs/promises";

export default () => {
  const files = await fs.readdir(".");
  return new Response(JSON.stringify(files), {
    headers : {"content-type" : "application/json"},
  });
};

export const config = {
  path : "/"
};
