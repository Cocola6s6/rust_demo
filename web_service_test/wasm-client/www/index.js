import * as wasm from "wasm-client";

wasm.greet();

const myForm = document.getElementById("myForm");
myForm.addEventListener("submit", e => {
    e.preventDefault;
    const courseName = document.getElementById("courseName").value;
    const teacherId = document.getElementById("teacherId").value;

    wasm.insert_course_api(teacherId, courseName);
});