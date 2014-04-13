import std.stdio;

void main()
{
    uint count = 1;
    double secondsAgo = 60 * 60 * 24 * count * -1;
    writeln("seconds ago:", secondsAgo);
}
