library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.array_type_pkg.all;

entity run_test is
end entity run_test;

architecture behaviour of run_test is

    component implementation is
        port(
            clk, rst : in std_logic;
            offline : in std_logic;
            input_time : in std_logic_vector(63 downto 0);
            new_input: in std_logic;
			a_data_in : in std_logic_vector(7 downto 0);
			a_data_in_new_input : in std_logic;
			b_data_in : in std_logic_vector(7 downto 0);
			b_data_in_new_input : in std_logic;
            time_stream : out std_logic_vector(63 downto 0);
			a_stream : out std_logic_vector(7 downto 0);
			b_stream : out std_logic_vector(7 downto 0);
			c_stream : out std_logic_vector(7 downto 0);
			d_stream : out std_logic_vector(7 downto 0);
			e_stream : out std_logic_vector(7 downto 0);
			f_stream : out std_logic_vector(7 downto 0);
			g_stream : out std_logic_vector(7 downto 0);
			time_stream_stream : out std_logic_vector(7 downto 0);
            lost_data : out std_logic
        );
    end component;

    signal clk : std_logic := '0';
    signal rst : std_logic := '1';
    signal time_data : std_logic_vector(63 downto 0) := (others => '0');
    signal time_stream : std_logic_vector(63 downto 0);
	signal a_data : std_logic_vector(7 downto 0) := (others => '0');
	signal a_data_new_input : std_logic := '0';
	signal a_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal b_data : std_logic_vector(7 downto 0) := (others => '0');
	signal b_data_new_input : std_logic := '0';
	signal b_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal c_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal d_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal e_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal f_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal g_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal time_stream_data_out : std_logic_vector(7 downto 0) := (others => '0');
    signal new_input_in : std_logic := '0';
    signal lost_data : std_logic := '0';

    -- constants
    signal const1 : std_logic := '1';

    -- config signals
    constant new_input_clock_cycle : natural := 10;
    constant number_iterations : natural := 2*20;
    constant number_inputs : natural := 7;

    -- set inputs
    -- current time unit: nanoseconds -> time given in milliseconds
    constant time_unit_multiplication : unsigned(31 downto 0) := to_unsigned(1000000, 32);
	constant time_test_data : unsigned32_array(0 to 20) := (
		to_unsigned(1100,32),
		to_unsigned(1110, 32),
		to_unsigned(1120, 32),
		to_unsigned(1130, 32),
		to_unsigned(2100, 32),
		to_unsigned(2200, 32),
		to_unsigned(2300, 32),
		to_unsigned(2500, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32)
	);
	constant new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant a_test_data : signed8_array(0 to 20) := (
		to_signed(4, a_data'length),
		to_signed(7, a_data'length),
		to_signed(9, a_data'length),
		to_signed(8, a_data'length),
		to_signed(1, a_data'length),
		to_signed(5, a_data'length),
		to_signed(7, a_data'length),
		to_signed(9, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length)
	);
	constant a_new_input_test_data : bit_array(0 to 20) := ('1','1','1','0','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant b_test_data : signed8_array(0 to 20) := (
		to_signed(3, b_data'length),
		to_signed(13, b_data'length),
		to_signed(2, b_data'length),
		to_signed(5, b_data'length),
		to_signed(8, b_data'length),
		to_signed(9, b_data'length),
		to_signed(0, b_data'length),
		to_signed(10, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length)
	);
	constant b_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','0','0','1','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant exp_c_data : signed8_array(0 to 20) := (
		to_signed(10, c_data_out'length),
		to_signed(27, c_data_out'length),
		to_signed(28, c_data_out'length),
		to_signed(28, c_data_out'length),
		to_signed(22, c_data_out'length),
		to_signed(22, c_data_out'length),
		to_signed(22, c_data_out'length),
		to_signed(58, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length)
	);
	constant exp_d_data : signed8_array(0 to 20) := (
		to_signed(7, d_data_out'length),
		to_signed(17, d_data_out'length),
		to_signed(13, d_data_out'length),
		to_signed(13, d_data_out'length),
		to_signed(39, d_data_out'length),
		to_signed(39, d_data_out'length),
		to_signed(39, d_data_out'length),
		to_signed(45, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length)
	);
	constant exp_e_data : signed8_array(0 to 20) := (
		to_signed(11, e_data_out'length),
		to_signed(31, e_data_out'length),
		to_signed(35, e_data_out'length),
		to_signed(35, e_data_out'length),
		to_signed(31, e_data_out'length),
		to_signed(31, e_data_out'length),
		to_signed(31, e_data_out'length),
		to_signed(65, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length),
		to_signed(0, e_data_out'length)
	);
	constant exp_f_data : signed8_array(0 to 20) := (
		to_signed(4, f_data_out'length),
		to_signed(11, f_data_out'length),
		to_signed(16, f_data_out'length),
		to_signed(16, f_data_out'length),
		to_signed(10, f_data_out'length),
		to_signed(6, f_data_out'length),
		to_signed(12, f_data_out'length),
		to_signed(16, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length),
		to_signed(0, f_data_out'length)
	);
	constant exp_g_data : signed8_array(0 to 20) := (
		to_signed(7, g_data_out'length),
		to_signed(24, g_data_out'length),
		to_signed(18, g_data_out'length),
		to_signed(18, g_data_out'length),
		to_signed(18, g_data_out'length),
		to_signed(18, g_data_out'length),
		to_signed(18, g_data_out'length),
		to_signed(26, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length),
		to_signed(0, g_data_out'length)
	);
	constant exp_time_stream_data : signed8_array(0 to 20) := (
		to_signed(6, time_stream_data_out'length),
		to_signed(6, time_stream_data_out'length),
		to_signed(6, time_stream_data_out'length),
		to_signed(6, time_stream_data_out'length),
		to_signed(9, time_stream_data_out'length),
		to_signed(9, time_stream_data_out'length),
		to_signed(9, time_stream_data_out'length),
		to_signed(9, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length),
		to_signed(0, time_stream_data_out'length)
	);

begin

    implementation_instance: implementation
        port map (
            clk => clk,
            rst => rst,
            offline => const1,
            input_time => time_data,
            new_input => new_input_in,
					a_data_in => a_data,
					a_data_in_new_input => a_data_new_input,
					b_data_in => b_data,
					b_data_in_new_input => b_data_new_input,
            time_stream => time_stream,
					a_stream => a_data_out,
					b_stream => b_data_out,
				c_stream => c_data_out,
				d_stream => d_data_out,
				e_stream => e_data_out,
				f_stream => f_data_out,
				g_stream => g_data_out,
				time_stream_stream => time_stream_data_out,
            lost_data => lost_data
        );

    process

    begin
        -- reset to get initial values
        rst <= '0';
        wait for 1 ps;
        -- set number of repetitions
        for I in 1 to new_input_clock_cycle loop
            clk <= '1';
            wait for 1 ps;
            clk <= '0';
            wait for 1 ps;
        end loop;
        -- set number of repetitions
        for I in 0 to 2*number_inputs loop
            --set inputs for each iteration
            if (I <= number_inputs) then
				time_data <= std_logic_vector(time_test_data(I) * time_unit_multiplication);
				new_input_in <= new_input_test_data(I);
				a_data <= std_logic_vector(a_test_data(I));
				a_data_new_input <= a_new_input_test_data(I);
				b_data <= std_logic_vector(b_test_data(I));
				b_data_new_input <= b_new_input_test_data(I);
            end if;
            for I_inner in 1 to number_iterations loop
                clk <= '1';
                wait for 1 ps;
                clk <= '0';
                wait for 1 ps;
            end loop;
            new_input_in <= '0';
			a_data_new_input <= '0';
			b_data_new_input <= '0';
            for I_inner in 1 to 20*number_iterations loop
                clk <= '1';
                wait for 1 ps;
                clk <= '0';
                wait for 1 ps;
            end loop;
            if (I <= number_inputs) then
				assert a_data_out = a_data
					report "a_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert b_data_out = b_data
					report "b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert c_data_out = std_logic_vector(exp_c_data(I))
					report "c_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert d_data_out = std_logic_vector(exp_d_data(I))
					report "d_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert e_data_out = std_logic_vector(exp_e_data(I))
					report "e_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert f_data_out = std_logic_vector(exp_f_data(I))
					report "f_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert g_data_out = std_logic_vector(exp_g_data(I))
					report "g_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert time_stream_data_out = std_logic_vector(exp_time_stream_data(I))
					report "time_stream_data differ at iteration I = " & integer'image(I)
					severity Error;
            end if;
        end loop;
        wait;
    end process;

end behaviour;
