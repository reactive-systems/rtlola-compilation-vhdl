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
		a_data_in : in signed(31 downto 0);
		a_en_in : in std_logic;
		a_data_out : out signed(31 downto 0);
		a_en_out : out std_logic;
		s_s_en_out : out std_logic;
		c_s_en_out : out std_logic;
		av_s_en_out : out std_logic;
		a_u_en_out : out std_logic;
		s_u_en_out : out std_logic;
		c_u_en_out : out std_logic;
		av_u_en_out : out std_logic;
        time_out : out unsigned(63 downto 0);
        push_out : out std_logic
    );
end hlQInterface;

--* Streams and their Activation Conditions:
--* Input Streams 
--* - a @ { a }
--* Event-based Output Streams 
--* - a_u @ { a }
--* Periodic Output Streams 
--* - s_s @ 10Hz
--* - c_s @ 10Hz
--* - av_s @ 10Hz
--* - s_u @ 10Hz
--* - c_u @ 10Hz
--* - av_u @ 10Hz
--* Resulting Deadline Array
--* || s_s, c_s, av_s, s_u, c_u, av_u ||

architecture behavioral of hlQInterface is

    -- Internal Signal Declarations
    signal clock_state_machine : integer;
    signal push_to_queue : std_logic;
	signal a_data_push : signed(31 downto 0);
	signal a_en_push: std_logic;
	signal s_s_en_push : std_logic;
	signal c_s_en_push : std_logic;
	signal av_s_en_push : std_logic;
	signal a_u_en_push : std_logic;
	signal s_u_en_push : std_logic;
	signal c_u_en_push : std_logic;
	signal av_u_en_push : std_logic;
	signal s_s_en_array : bit_array(0 downto 0);
	signal c_s_en_array : bit_array(0 downto 0);
	signal av_s_en_array : bit_array(0 downto 0);
	signal s_u_en_array : bit_array(0 downto 0);
	signal c_u_en_array : bit_array(0 downto 0);
	signal av_u_en_array : bit_array(0 downto 0);
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
			s_s_en_push <= '0';
			c_s_en_push <= '0';
			av_s_en_push <= '0';
			a_u_en_push <= '0';
			s_u_en_push <= '0';
			c_u_en_push <= '0';
			av_u_en_push <= '0';
            -- Initialize Deadline Arrays
            --* Deadline Array
            --* || s_s, c_s, av_s, s_u, c_u, av_u ||
			s_s_en_array(0) <= '1';
			c_s_en_array(0) <= '1';
			av_s_en_array(0) <= '1';
			s_u_en_array(0) <= '1';
			c_u_en_array(0) <= '1';
			av_u_en_array(0) <= '1';
        elsif (rising_edge(clk)) then
            clock_state_machine <= (clock_state_machine + 1) mod 4;
            if push_deadline = '1' and clock_state_machine = 0 then
                -- Deadline Handling
                push_to_queue <= '1';
                last_deadline_id <= (last_deadline_id + 1) mod 1;
                time_to_queue <= time_for_deadline;
				--* a @ { a }
				a_en_push <= '0';
				--* a_u @ { a }
				a_u_en_push <= '0';
				--* s_s @ 10Hz
				s_s_en_push <= s_s_en_array(last_deadline_id);
				--* c_s @ 10Hz
				c_s_en_push <= c_s_en_array(last_deadline_id);
				--* av_s @ 10Hz
				av_s_en_push <= av_s_en_array(last_deadline_id);
				--* s_u @ 10Hz
				s_u_en_push <= s_u_en_array(last_deadline_id);
				--* c_u @ 10Hz
				c_u_en_push <= c_u_en_array(last_deadline_id);
				--* av_u @ 10Hz
				av_u_en_push <= av_u_en_array(last_deadline_id);
            elsif push_event = '1' and clock_state_machine = 2 then
                -- Event Handling
                push_to_queue <= '1';
                time_to_queue <= time_for_event;
				--* a @ { a }
				a_data_push <= a_data_in;
				a_en_push <= a_en_in;
				--* a_u @ { a }
				a_u_en_push <= '1' and a_en_in;
				--* s_s @ 10Hz
				s_s_en_push <= '0';
				--* c_s @ 10Hz
				c_s_en_push <= '0';
				--* av_s @ 10Hz
				av_s_en_push <= '0';
				--* s_u @ 10Hz
				s_u_en_push <= '0';
				--* c_u @ 10Hz
				c_u_en_push <= '0';
				--* av_u @ 10Hz
				av_u_en_push <= '0';
            else
                -- Enable No Stream
                push_to_queue <= '0';
				a_en_push <= '0';
				s_s_en_push <= '0';
				c_s_en_push <= '0';
				av_s_en_push <= '0';
				a_u_en_push <= '0';
				s_u_en_push <= '0';
				c_u_en_push <= '0';
				av_u_en_push <= '0';
            end if;
        end if;
    end process;

    push_out <= push_to_queue;
    time_out <= time_to_queue;
	a_data_out <= a_data_push;
	a_en_out <= a_en_push;
	s_s_en_out <= s_s_en_push;
	c_s_en_out <= c_s_en_push;
	av_s_en_out <= av_s_en_push;
	a_u_en_out <= a_u_en_push;
	s_u_en_out <= s_u_en_push;
	c_u_en_out <= c_u_en_push;
	av_u_en_out <= av_u_en_push;

end behavioral;
