<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  let selectedPictures: any = [];
  let pictures: any[] = [];
  let error = "";
  let success = false;

  /**
   * Add Picture to be shown
   * Read in Uint8Array from tauri and convert it into a blob
   * Add it to the pictures array
   * @param picture
   */
  async function addPicture(picture: any) {
    selectedPictures = [...selectedPictures, picture];

    // Uint8Array
    const contents = await readBinaryFile(picture);
    const fileName = picture.split("\\").pop();
    const type = picture.split(".").pop();
    const blob = new Blob([contents], { type: type });

    const url = URL.createObjectURL(blob);

    pictures = [...pictures, url];
  }

  /**
   * Open File Dialog
   */
  async function openFileDialog() {
    success = false;
    error = "";
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpeg"],
        },
      ],
    });

    if (Array.isArray(selected)) {
      selected.map((url: any) => addPicture(url));
    } else if (selected === null) {
      return;
    } else {
      addPicture(selected);
    }
  }

  /**
   * Delete Picture from the pictures array
   * @param pictureId
   */
  function deletePicture(pictureId: number) {
    pictures = pictures.filter((url, i) => i !== pictureId);
    selectedPictures = selectedPictures.filter((url, i) => i !== pictureId);
  }

  /**
   * Create the command for the denoiser
   */

 
  let args: any[string] = [];

  $: {
    let cmd: any[string] = [];

    cmd.push("-i");

    selectedPictures.map((url: any) => {
    cmd.push('"' + url + '"');
    });

    cmd.push("-o");

    selectedPictures.map((url: any) => {
      let file_extension = url.split(".").pop();
      // Removing the file extension from the PATH
      let filePath = url.slice(0, -(file_extension.length + 1));
      cmd.push('"' + filePath + "_denoised." + file_extension + '"');
    });

    args = cmd;
  }

  /**
   * Handle the denoising
   * Invoke the tauri command
   */
  async function handleDenoise() {
    invoke("run_denoiser", { args: args })
      .then((e: any) => {
        pictures = [];
        selectedPictures = [];
        success = true;
        console.log(e);
      })
      .catch((e: any) => {
        pictures = [];
        selectedPictures = [];
        success = false;
        error = e;
      });
  }
</script>

<h1 class="flex justify-center text-4xl font-bold text-white">
  Optix Image Denoiser
</h1>

<div class="mx-auto flex justify-center mt-4 gap-2">
  <button class="btn btn-primary" on:click={openFileDialog}>Choose Files</button
  >
  <button class="btn btn-success" on:click={handleDenoise}
    >Denoise Selected Pictures</button
  >
</div>

<div class="mt-10">
  <div class="grid grid-cols-4 gap-4">
    {#each pictures as url, i (i)}
      <div class="flex">
        <button
          class="absolute cursor-pointer"
          on:click={() => deletePicture(i)}
          ><i class="bi bi-trash-fill text-error text-4xl" /></button
        >
        <!-- svelte-ignore a11y-img-redundant-alt -->
        <img src={url} class="max-w-sm rounded-2xl" alt="chosen image" />
      </div>
    {/each}
  </div>
</div>

{#if success}
  <h1 class="text-success font-bold flex justify-center">Success</h1>
{/if}

{#if error}
  <h1 class="text-error font-bold flex justify-center">{error}</h1>
{/if}
