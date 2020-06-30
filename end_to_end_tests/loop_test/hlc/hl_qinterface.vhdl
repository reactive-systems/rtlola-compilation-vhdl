library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;
use work.array_type_pkg.all;

entity hlQInterface is
    port (
        clk, rst : in std_logic;
        time_for_event : in unsigned(63 downto 0);
        time_for_deadline : in unsigned(63 downto 0);
        push_event : std_logic;
        push_deadline : in std_logic;
		a_data_in : in signed(7 downto 0);
		a_en_in : in std_logic;
		b_data_in : in signed(7 downto 0);
		b_en_in : in std_logic;
		a_data_out : out signed(7 downto 0);
		a_en_out : out std_logic;
		b_data_out : out signed(7 downto 0);
		b_en_out : out std_logic;
		c_en_out : out std_logic;
		d_en_out : out std_logic;
		e_en_out : out std_logic;
		f_en_out : out std_logic;
		g_en_out : out std_logic;
		time_stream_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* - b @ { b }
--* Event-based Output Streams 
--* - c @ { a, b }
--* - d @ { a, b }
--* - e @ { a, b }
--* - f @ { a }
--* - g @ { a, b }
--* Periodic Output Streams 
--* - time_stream @ 1Hz
--* Resulting Deadline Array
--* || time_stream ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : signed(7 downto 0);
	signal a_en_push: std_logic;
	signal b_data_push : signed(7 downto 0);
	signal b_en_push: std_logic;
	signal c_en_push : std_logic;
	signal d_en_push : std_logic;
	signal e_en_push : std_logic;
	signal f_en_push : std_logic;
	signal g_en_push : std_logic;
	signal time_stream_en_push : std_logic;
	signal time_stream_en_array : bit_array(0 downto 0);
    signal last_deadline_id : integer;
    signal time_to_queue : unsigned(63 downto 0);

begin

    process(clk, rst) begin
        if (rst = '1') then
            -- Reset Phase
            clock_state_machine <= 0;
            time_to_queue <= (others => '0');
            push_to_queue <= '0';
            last_deadline_id <= 0;
			a_data_push <= (others => '0');
			a_en_push <= '0';
			b_data_push <= (others => '0');
			b_en_push <= '0';
			c_en_push <= '0';
			d_en_push <= '0';
			e_en_push <= '0';
			f_en_push <= '0';
			g_en_push <= '0';
			time_stream_en_push <= '0';
            -- Initialize Deadline Arrays
            --* Deadline Array
            --* || time_stream ||
			time_stream_en_array(0) <= '1';
        elsif (rising_edge(clk)) then
            clock_state_machine <= (clock_state_machine + 1) mod 4;
            if push_deadline = '1' and clock_state_machine = 0 then
                -- Deadline Handling
                push_to_queue <= '1';
                last_deadline_id <= (last_deadline_id + 1) mod 1;
                time_to_queue <= time_for_deadline;
				--* a @ { a }
				a_en_push <= '0';
				--* b @ { b }
				b_en_push <= '0';
				--* c @ { a, b }
				c_en_push <= '0';
				--* d @ { a, b }
				d_en_push <= '0';
				--* e @ { a, b }
				e_en_push <= '0';
				--* f @ { a }
				f_en_push <= '0';
				--* g @ { a, b }
				g_en_push <= '0';
				--* time_stream @ 1Hz
				time_stream_en_push <= time_stream_en_array(last_deadline_id);
            elsif push_event = '1' and clock_state_machine = 2 then
                -- Event Handling
                push_to_queue <= '1';
                time_to_queue <= time_for_event;
				--* a @ { a }
				a_data_push <= a_data_in;
				a_en_push <= a_en_in;
				--* b @ { b }
				b_data_push <= b_data_in;
				b_en_push <= b_en_in;
				--* c @ { a, b }
				c_en_push <= '1' and a_en_in and b_en_in;
				--* d @ { a, b }
				d_en_push <= '1' and a_en_in and b_en_in;
				--* e @ { a, b }
				e_en_push <= '1' and a_en_in and b_en_in;
				--* f @ { a }
				f_en_push <= '1' and a_en_in;
				--* g @ { a, b }
				g_en_push <= '1' and a_en_in and b_en_in;
				--* time_stream @ 1Hz
				time_stream_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				b_en_push <= '0';
				c_en_push <= '0';
				d_en_push <= '0';
				e_en_push <= '0';
				f_en_push <= '0';
				g_en_push <= '0';
				time_stream_en_push <= '0';
            end if;
        end if;
    end process;

    push_out <= push_to_queue;
    time_out <= time_to_queue;
	a_data_out <= a_data_push;
	a_en_out <= a_en_push;
	b_data_out <= b_data_push;
	b_en_out <= b_en_push;
	c_en_out <= c_en_push;
	d_en_out <= d_en_push;
	e_en_out <= e_en_push;
	f_en_out <= f_en_push;
	g_en_out <= g_en_push;
	time_stream_en_out <= time_stream_en_push;

end behavioral;
