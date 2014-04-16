/*
compile with mcs file.cs
run with mono file.exe
*/

public class Unsigned
{
    public static void Main()
    {
        ulong count = 1;
        double secondsAgo = 60 * 60 *24 * count * -1;
        System.Console.WriteLine("CSharp Seconds Ago:");
        System.Console.WriteLine(secondsAgo);
    }
}
