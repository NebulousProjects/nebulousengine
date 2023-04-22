Deno.core.print("Start!\n");

function update() {
    Deno.core.print("Update!\n");
}

export default {
    update: update
};