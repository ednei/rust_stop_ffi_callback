#include "../jigsaw/jigsaw.h"

extern "C" {
    Jigsaw *jigsaw__new(EscapeCallback callback){
        return new Jigsaw(callback);
    }

    void jigsaw__i_want_to_play_a_game(Jigsaw *jigsaw){
        jigsaw->i_want_to_play_a_game();
    }
}