import { RouteDataFunc } from '@solidjs/router';
import { Resource, createResource } from 'solid-js';
import { Upload } from '../api/uploads';

const UploadsData: RouteDataFunc<unknown, Resource<Upload[]>> = args => {
  // TODO: use API
  // const [data] = createResource(() => getUploads(args.params.id));
  const [data] = createResource<Upload[]>(async () => {
    const sleep = (ms: number) =>
      new Promise(resolve => setTimeout(resolve, ms));
    await sleep(500);
    return Array.from({ length: 20 }, (_, i) => ({
      id: `${i}`,
      name: `Upload ${i}`,
      description: `Lorem ipsum, dolor sit amet consectetur adipisicing elit. Laboriosam dignissimos voluptatem consequuntur fuga placeat, atque mollitia autem iusto voluptatum nulla molestiae a architecto, sint alias ipsam id voluptates! Ipsam fugit voluptas molestiae suscipit impedit sit assumenda id voluptates natus sequi atque aliquid inventore tempora repudiandae labore quo, nam mollitia sapiente.`,
      price: i,
      uploader: `Uploader ${i}`,
      upload_date: `${i % 3}`,
      last_modified_date: `${i % 3}`,
      belongs_to: `${i % 3}`,
      held_by: `${i % 3}`,
    }));
  });
  return data;
};

export default UploadsData;
export type UploadsDataType = typeof UploadsData;
