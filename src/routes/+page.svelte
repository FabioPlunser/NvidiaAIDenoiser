<script lang="ts">
  // Import necessary modules
  import { open } from "@tauri-apps/api/dialog";
  import { readBinaryFile } from "@tauri-apps/api/fs";
  import { invoke } from "@tauri-apps/api/tauri";

  // Initialize variables
  let selectedPictures: any = [];
  let pictures: any[] = [];
  let error = "";
  let msg = "";
  let success = false;
  let loadingDenoiser = false;
  let iterations = 1;

  /**
   * Add Picture to be shown
   * Read in Uint8Array from tauri and convert it into a blob
   * Add it to the pictures array
   * @param picture
   */
  async function convertPicture(picture: any) {
    // Read the image file as Uint8Array
    const contents = await readBinaryFile(picture);

    // Get the file name and type/extension
    const fileName = picture.split("\\").pop();
    const type = picture.split(".").pop();

    // Create a Blob from the Uint8Array data
    const blob = new Blob([contents], { type: type });

    // Return the URL of the created Blob
    return URL.createObjectURL(blob);
  }

  // Function to add a selected picture to the selectedPictures array
  function addPicture(picture: any) {
    selectedPictures = [...selectedPictures, picture];
  }

  /**
   * Open File Dialog
   */
  async function openFileDialog() {
    success = false;
    error = "";
    // Open a file dialog and allow multiple image selection with specific extensions
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Image",
          extensions: ["png", "jpg", "jpeg", "svg"],
        },
      ],
    });

    // Handle the selected images
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
    // Filter out the specified picture from both selectedPictures and pictures arrays
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

      // Prepare the denoising command with input and output file paths
      let cmd: any[string] = [];
      cmd.push("-i");
      cmd.push(picture);
      cmd.push("-o");
      let file_extension = picture.split(".").pop();
      // Removing the file extension from the PATH
      let filePath = picture.slice(0, -(file_extension.length + 1));
      cmd.push(filePath + "_denoised." + file_extension);
      cmd.push("-repeat " + iterations);

      // Remove the first picture from the selectedPictures array
      selectedPictures = selectedPictures.filter((url, i) => i !== 0);

      // Invoke the "run_denoiser" command and handle the result
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

      // Loop will continue to denoise the rest of the images, but only update the last result.
      // If you want individual denoising updates, you may need to change the code accordingly.
    }
  }
</script>


<h1 class="flex justify-center text-4xl font-bold text-white">
  Optix Image Denoiser
</h1>

<div class="mx-auto flex justify-center mt-4 gap-2">
  <button class="btn btn-primary" on:click={openFileDialog}>Choose Files</button
  >
  <label class="flex my-auto justify-center items-center">
    <h1 class="font-bold text-lg mr-4">Iterations: </h1>
    <input class="input input-bordered input-secondary text-white" type="number" placeholder="1" bind:value={iterations} min="1" max="200"/>
  </label>
  <button disabled='{selectedPictures.length <= 0}' class="btn btn-success" on:click={handleDenoise}
    >Denoise Selected Pictures</button
  >
</div>

<div class="mt-10">
  <div class="grid grid-rows md:grid-cols-2 xl:grid-cols-4 gap-4">
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
