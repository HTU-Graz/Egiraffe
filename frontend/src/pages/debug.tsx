import { Link, useRouteData } from '@solidjs/router';
import { For, Suspense, createSignal } from 'solid-js';
import { Course, GetCoursesResponse } from '../api/courses';
import { Upload, UploadRequest, upload } from '../api/uploads';
import { getUniversities } from '../api/universities';
import { put } from '../api';

const unis = await getUniversities();
const tu_graz = unis.find((uni) => uni.short_name === "TUG")!;

export default function Debug() {

  const [course, setCourse] = createSignal<Course | null>(null);
  const [_upload, setUpload] = createSignal<Upload | null>(null);

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

    console.debug("Creating course");

    const json = await put("/api/v1/mod/courses/create", {
      "name": "Introduction to Egiraffe",
      "held_at": tu_graz.id,
    } as Course);

    console.debug("Created course", json);

    const courses = (await put<GetCoursesResponse>("/api/v1/get/courses", {})).courses as Course[];
    setCourse(courses[0]);

    console.debug("Course is:", courses[0])

    console.log(json);
  }

  const handle_create_upload = async (e: Event) => {
    e.preventDefault();
    const upload_ = await upload({
      belongs_to: course()?.id,
      name: "Test upload",
      description: "This is a test upload",
      price: 0,
    })

    setUpload(upload_);
  }

  return (
    <div>
      <h1 class="text-3xl font-bold">
        Debug page
      </h1>
      <ol class="list-decimal">
        <li>Log in as Mod or Admin</li>
        <li>Click Create course</li>
        <li>Click Create upload</li>
        <li>Select a file</li>
        <li>Click yeet</li>
        <li>Check JS & backend consoles</li>
      </ol>

      <br />

      <p>
        Currently targeting "{course()?.name}" ({course()?.id}).
      </p>
      <p>
        Upload is "{_upload()?.name}" ({_upload()?.id}).
      </p>

      <br />

      <button onClick={handle_create_course} class="btn btn-accent">Create course</button>
      &nbsp;&nbsp;&nbsp;
      <button onClick={handle_create_upload} class="btn btn-accent">Create upload</button>

      <br /> <br />

      <form id="upload-form">
        <input type="file" name="file" />
        <input type="text" name="upload_id" value={_upload()?.id} hidden />
        <br />
        <br />
        <button type="submit" onClick={handle_upload} class="btn btn-accent">Yeet</button>
      </form>

    </div>
  );
}
