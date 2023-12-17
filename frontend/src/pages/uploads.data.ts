import { RouteDataFunc } from "@solidjs/router";
import { Resource, createResource } from "solid-js";
import { Upload, getUploads } from "../api/uploads";

const UploadsData: RouteDataFunc<unknown, Resource<Upload[]>> = (args) => {
  const [data] = createResource<Upload[]>(async () => await getUploads(args.params.id));
  return data;
};

export default UploadsData;
export type UploadsDataType = typeof UploadsData;
