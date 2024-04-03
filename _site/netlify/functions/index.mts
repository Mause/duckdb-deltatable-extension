import fs from "fs/promises";

export default async () => {
  return new Response(JSON.stringify({
	  files: await fs.readdir("."),
	  parent: await fs.readdir(".."),
	  grandparent: await fs.readdir("../.."),
  }), {
    headers : {"content-type" : "application/json"},
  });
};

export const config = {
  path : "/"
};
