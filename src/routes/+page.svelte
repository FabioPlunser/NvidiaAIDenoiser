<script lang="ts">
  import { writeFile, writeBinaryFile, readBinaryFile, BaseDirectory} from '@tauri-apps/api/fs';
  import { invoke  } from '@tauri-apps/api/tauri';

  let imageUrls: any[] = [];
  let imageNames: string[] = [];

  function handleFileSelected(event: any) {
    event.preventDefault();
    const files = event.target.files;
    for (let i= 0; i < files.length; i++){
      const file = files[i]   

      handleTempPicture(file)
      imageNames = [...imageNames, file.name.replace(" ", "_")];

      const reader = new FileReader();
      reader.onload = (e: any) => {
        imageUrls = [...imageUrls, e.target.result];
      }
      reader.readAsDataURL(file);
    }
  }

  async function handleTempPicture(picture: any) {
    const arrayBuffer = await picture.arrayBuffer();
    await writeBinaryFile(picture.name.replace(" ", "_"), new Uint8Array(arrayBuffer));
  }


  function deletePicture(pictureId: number){
    imageUrls = imageUrls.filter((url, i) => i !== pictureId);
  }

  $: inPictures = "./" + imageNames.join(" ./");
  $: outPicturs = inPictures.replace(/\.png/g, "_denoised.png");

  $: cmd = "-i " + inPictures + " -o " + outPicturs;
  $: console.log(cmd)

  async function handleDenoise(){
    const result = await invoke('run_denoiser', {cmd});
  }
 
 
</script>

<h1 class="flex justify-center text-4xl font-bold text-white">
  Optix Image Denoiser
</h1>


<div class="mx-auto flex justify-center mt-4 gap-2">
  <input
    multiple={true}
    type="file"
    class="file-input w-full ile-input-bordered file-input-primary max-w-xs"
    accept="image/gif, image/jpeg, image/png"
    on:change={handleFileSelected}
  />
  <button class="btn btn-success" on:click={handleDenoise}>Denoise Selected Pictures</button>
</div>

<div class="mt-10">
    <div class="grid grid-cols-4 gap-4">
      {#each imageUrls as url, i (i)}
        <div class="flex">
          <button class="absolute cursor-pointer" on:click={()=>deletePicture(i)}><i class="bi bi-trash-fill text-error text-4xl"></i></button>
          <img src={url} class="max-w-md rounded-2xl" alt="chosen image" />
        </div>
      {/each}
    </div>
</div>