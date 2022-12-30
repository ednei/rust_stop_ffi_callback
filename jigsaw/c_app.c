#include <stdio.h>

//https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
typedef struct Opaque Jigsaw;
Jigsaw *jigsaw__new(void (*escape_callback)(void));
void jigsaw__i_want_to_play_a_game(Jigsaw *jigsaw);

void escape(){
    printf("Stupid guy on C: Well, I just do the stupid thing JigSaw expect me to do!\n");
}

int main(int argc, char *argv[]) {
    Jigsaw *jigsaw;
    jigsaw = jigsaw__new(&escape);
    jigsaw__i_want_to_play_a_game(jigsaw);
}