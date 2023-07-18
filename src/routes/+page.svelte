<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  let selectedPictures: any = [];
  let pictures: any[] = [];
  let error = "";
  let msg = "";
  let success = false;
  let loadingDenoiser = false;

  /**
   * TODO Put this into a array and await the blob creation int the array to show the picture is loading
   * Add Picture to be shown
   * Read in Uint8Array from tauri and convert it into a blob
   * Add it to the pictures array
   * @param picture
   */
  async function convertPicture(picture: any) {
    // selectedPictures = [...selectedPictures, picture];

    // Uint8Array
    const contents = await readBinaryFile(picture);
    const fileName = picture.split("\\").pop();
    const type = picture.split(".").pop();
    const blob = new Blob([contents], { type: type });

    return URL.createObjectURL(blob);

    // pictures = [...pictures, url];
  }

  function addPicture(picture: any) {
    selectedPictures = [...selectedPictures, picture];
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
   * Handle the denoising
   * Invoke the tauri command
   */
  async function handleDenoise() {
    for (let picture of selectedPictures) {
      loadingDenoiser = true;

      let cmd: any[string] = [];
      cmd.push("-i");
      cmd.push(picture);
      cmd.push("-o");
      let file_extension = picture.split(".").pop();
      // Removing the file extension from the PATH
      let filePath = picture.slice(0, -(file_extension.length + 1));
      cmd.push(filePath + "_denoised." + file_extension);

      selectedPictures = selectedPictures.filter((url, i) => i !== 0);

      invoke("run_denoiser", { args: cmd })
        .then((e: any) => {
          success = true;
          msg = e;
          loadingDenoiser = false;
        })
        .catch((e: any) => {
          success = false;
          error = e;
          loadingDenoiser = false;
        });
    }
    
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
    {#each selectedPictures as picture, i (i)}
      {#await convertPicture(picture)}
      <span class="loading loading-bars loading-lg text-success"></span>
      {:then url}
        <div class="flex">
          <button
            class="absolute cursor-pointer"
            on:click={() => deletePicture(i)}
            ><i class="bi bi-trash-fill text-error text-4xl" /></button
          >
          <!-- svelte-ignore a11y-img-redundant-alt -->
          <img src={url} class="max-w-sm rounded-2xl" alt="chosen image" />
        </div>
      {:catch error}
        <p class="text-error">{error}</p>
      {/await}
    {/each}
  </div>
</div>

{#if loadingDenoiser}
  <div class="flex justify-center mx-auto">
    <span class="loading loading-dots text-info loading-lg" />
  </div>
{/if}

{#if success}
  <h1 class="text-success font-bold flex justify-center">Success</h1>
  <!-- <h1 class="text-white font-bold flex justify-center">{msg}</h1> -->
{/if}

{#if error}
  <h1 class="text-error font-bold flex justify-center">{error}</h1>
{/if}
