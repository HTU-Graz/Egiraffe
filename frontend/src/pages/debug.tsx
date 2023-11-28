import { Link, useRouteData } from '@solidjs/router';
import { For, Suspense } from 'solid-js';
import { Course } from '../api/courses';
import { CoursesDataType } from './courses.data';
import { UploadRequest } from '../api/uploads';

export default function Debug() {
  const handle_upload = async (e: Event) => {
    e.preventDefault();
    const form = document.getElementById("upload-form") as HTMLFormElement;
    const data = new FormData(form);
    const response = await fetch("/api/v1/do/file", {
      method: "PUT",
      body: data,
    });
    const json = await response.json();
    console.log(json);
  }

  const handle_create_course = async (e: Event) => {
    e.preventDefault();
    const data = JSON.stringify({
      id: "foobar",
      name: "foobar",
      held_at: "foobar",
    } as Course);

    const response = await fetch("/api/v1/mod/courses/create", {
      method: "PUT",
      body: data,
    });
    const json = await response.json();
    console.log(json);
  }

  const handle_create_upload = async (e: Event) => {
    e.preventDefault();
    const data = JSON.stringify({
      id: "foobar",
      name: "foobar",
      filename: "foobar",
      content_type: "foobar",
    } as UploadRequest);

    const response = await fetch("/api/v1/do/upload", {
      method: "PUT",
      body: data,
    });
    const json = await response.json();
    console.log(json);
  }

  return (
    <div>
      <button onClick={handle_create_course} class="btn btn-accent">Create course</button>
      &nbsp;&nbsp;&nbsp;
      <button onClick={handle_create_upload} class="btn btn-accent">Create upload</button>

      <br /> <br /> <br />

      <form id="upload-form">
        {/* <input type="text" name="name" id="name " value={"name_foobar"} /> */}
        {/* <input type="text" name="filename" id="filename " value={"filename_foobar"} /> */}
        {/* <input type="text" name="content_type" id="content_type " value={"content_type_foobar"} /> */}
        <input type="file" name="file" />
        <input type="text" name="upload_id" value="hello world" hidden />
        <br />
        <br />
        <button type="submit" onClick={handle_upload} class="btn btn-accent">Yeet</button>
      </form>


    </div>
  );
}
