<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { writeFile, writeBinaryFile, readBinaryFile, BaseDirectory, copyFile} from '@tauri-apps/api/fs';
  import { invoke  } from '@tauri-apps/api/tauri';
  import { Command } from "@tauri-apps/api/shell";
 


  // $: inPictures = "./" + imageNames.join(" ./");
  // $: outPicturs = inPictures.replace(/\.png/g, "_denoised.png");

  // $: cmd = "-i " + inPictures + " -o " + outPicturs;
  // $: console.log(cmd)

  

  let selectedPictures: any = []; 
  let pictures: any[] = []


  async function addPicture(picture: any){
    if(!selectedPictures.includes(picture)){
      console.log(picture);
      selectedPictures = [...selectedPictures, picture];
      console.log("selectedPictures", selectedPictures);

      // Uint8Array
      const contents = await readBinaryFile(picture);
      const fileName = picture.split("\\").pop();
      const outPath = "./" + fileName;
      const type = picture.split(".").pop();
      const blob = new Blob([contents], {type: type});

      const url = URL.createObjectURL(blob);
      
      pictures = [...pictures, url];
    
    }
  }

  async function openFileDialog(){
    const selected = await open({
        multiple: true,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpeg']
        }]
      });

    if (Array.isArray(selected)) {
      selected.map((url: any) => addPicture(url));
    } else if (selected === null) {
      return;
    } else {
      addPicture(selected);
    }
  }

  function deletePicture(pictureId: number){
    pictures = pictures.filter((url, i) => i !== pictureId);
  }

  $: console.log(pictures);
  $: console.log(selectedPictures);

  function handleDenoise(){
    const result = invoke("run_denoiser");
    console.log(result);
  }
</script>

<h1 class="flex justify-center text-4xl font-bold text-white">
  Optix Image Denoiser
</h1>


<div class="mx-auto flex justify-center mt-4 gap-2">
  <button class="btn btn-primary" on:click={openFileDialog}>Choose Files</button>
  <button class="btn btn-success" on:click={handleDenoise}>Denoise Selected Pictures</button>
</div>

<div class="mt-10">
    <div class="grid grid-cols-4 gap-4">
      {#each pictures as url, i (i)}
        <div class="flex">
          <button class="absolute cursor-pointer" on:click={()=>deletePicture(i)}><i class="bi bi-trash-fill text-error text-4xl"></i></button>
          <!-- svelte-ignore a11y-img-redundant-alt -->
          <img src="{url}" class="max-w-sm rounded-2xl" alt="chosen image" />
        </div>
      {/each}
    </div>
</div>