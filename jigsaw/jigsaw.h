
using EscapeCallback = void (*)();

class Jigsaw{
    private:
    EscapeCallback escapeCallback;

    public:
    Jigsaw(EscapeCallback callback);
    ~Jigsaw();
    void i_want_to_play_a_game();
};