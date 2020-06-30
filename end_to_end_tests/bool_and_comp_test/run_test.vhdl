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
			a_data_in : in std_logic;
			a_data_in_new_input : in std_logic;
			b_data_in : in std_logic;
			b_data_in_new_input : in std_logic;
			ID_data_in : in std_logic_vector(7 downto 0);
			ID_data_in_new_input : in std_logic;
            time_stream : out std_logic_vector(63 downto 0);
			a_stream : out std_logic;
			b_stream : out std_logic;
			ID_stream : out std_logic_vector(7 downto 0);
			eq_stream : out std_logic;
			lt_stream : out std_logic;
			le_stream : out std_logic;
			gt_stream : out std_logic;
			ge_stream : out std_logic;
			neq_stream : out std_logic;
			not_a_stream : out std_logic;
			a_and_b_stream : out std_logic;
			a_or_b_stream : out std_logic;
			a_impl_b_stream : out std_logic;
			a_equiv_b_stream : out std_logic;
			a_xor_b_stream : out std_logic;
			true_const_stream : out std_logic;
			time_stream_stream : out std_logic_vector(7 downto 0);
            lost_data : out std_logic
        );
    end component;

    signal clk : std_logic := '0';
    signal rst : std_logic := '1';
    signal time_data : std_logic_vector(63 downto 0) := (others => '0');
    signal time_stream : std_logic_vector(63 downto 0);
	signal a_data : std_logic := '0';
	signal a_data_new_input : std_logic := '0';
	signal a_data_out : std_logic := '0';
	signal b_data : std_logic := '0';
	signal b_data_new_input : std_logic := '0';
	signal b_data_out : std_logic := '0';
	signal ID_data : std_logic_vector(7 downto 0) := (others => '0');
	signal ID_data_new_input : std_logic := '0';
	signal ID_data_out : std_logic_vector(7 downto 0) := (others => '0');
	signal eq_data_out : std_logic := '0';
	signal lt_data_out : std_logic := '0';
	signal le_data_out : std_logic := '0';
	signal gt_data_out : std_logic := '0';
	signal ge_data_out : std_logic := '0';
	signal neq_data_out : std_logic := '0';
	signal not_a_data_out : std_logic := '0';
	signal a_and_b_data_out : std_logic := '0';
	signal a_or_b_data_out : std_logic := '0';
	signal a_impl_b_data_out : std_logic := '0';
	signal a_equiv_b_data_out : std_logic := '0';
	signal a_xor_b_data_out : std_logic := '0';
	signal true_const_data_out : std_logic := '0';
	signal time_stream_data_out : std_logic_vector(7 downto 0) := (others => '0');
    signal new_input_in : std_logic := '0';
    signal lost_data : std_logic := '0';

    -- constants
    signal const1 : std_logic := '1';

    -- config signals
    constant new_input_clock_cycle : natural := 10;
    constant number_iterations : natural := 2*40;
    constant number_inputs : natural := 5;

    -- set inputs
    -- current time unit: nanoseconds -> time given in milliseconds
    constant time_unit_multiplication : unsigned(31 downto 0) := to_unsigned(1000000, 32);
	constant time_test_data : unsigned32_array(0 to 20) := (
		to_unsigned(0000,32),
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
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32)
	);
	constant new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant a_test_data : bit_array(0 to 20) := (
		'0',
		'0',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant a_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant b_test_data : bit_array(0 to 20) := (
		'0',
		'1',
		'0',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant b_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant ID_test_data : signed8_array(0 to 20) := (
		to_signed(1, ID_data'length),
		to_signed(4, ID_data'length),
		to_signed(5, ID_data'length),
		to_signed(6, ID_data'length),
		to_signed(7, ID_data'length),
		to_signed(5, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length),
		to_signed(0, ID_data'length)
	);
	constant ID_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant exp_eq_data : bit_array(0 to 20) := (
		'0',
		'0',
		'1',
		'0',
		'0',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_lt_data : bit_array(0 to 20) := (
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_le_data : bit_array(0 to 20) := (
		'1',
		'1',
		'1',
		'0',
		'0',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_gt_data : bit_array(0 to 20) := (
		'0',
		'0',
		'0',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_ge_data : bit_array(0 to 20) := (
		'0',
		'0',
		'1',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_neq_data : bit_array(0 to 20) := (
		'1',
		'1',
		'0',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_not_a_data : bit_array(0 to 20) := (
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_a_and_b_data : bit_array(0 to 20) := (
		'0',
		'0',
		'0',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_a_or_b_data : bit_array(0 to 20) := (
		'0',
		'1',
		'1',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_a_impl_b_data : bit_array(0 to 20) := (
		'1',
		'1',
		'0',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_a_equiv_b_data : bit_array(0 to 20) := (
		'1',
		'0',
		'0',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_a_xor_b_data : bit_array(0 to 20) := (
		'0',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_true_const_data : bit_array(0 to 20) := (
		'1',
		'1',
		'1',
		'1',
		'1',
		'1',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0',
		'0'
	);
	constant exp_time_stream_data : signed8_array(0 to 20) := (
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
					ID_data_in => ID_data,
					ID_data_in_new_input => ID_data_new_input,
            time_stream => time_stream,
					a_stream => a_data_out,
					b_stream => b_data_out,
					ID_stream => ID_data_out,
				eq_stream => eq_data_out,
				lt_stream => lt_data_out,
				le_stream => le_data_out,
				gt_stream => gt_data_out,
				ge_stream => ge_data_out,
				neq_stream => neq_data_out,
				not_a_stream => not_a_data_out,
				a_and_b_stream => a_and_b_data_out,
				a_or_b_stream => a_or_b_data_out,
				a_impl_b_stream => a_impl_b_data_out,
				a_equiv_b_stream => a_equiv_b_data_out,
				a_xor_b_stream => a_xor_b_data_out,
				true_const_stream => true_const_data_out,
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
				a_data <= (a_test_data(I));
				a_data_new_input <= a_new_input_test_data(I);
				b_data <= (b_test_data(I));
				b_data_new_input <= b_new_input_test_data(I);
				ID_data <= std_logic_vector(ID_test_data(I));
				ID_data_new_input <= ID_new_input_test_data(I);
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
			ID_data_new_input <= '0';
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
				assert ID_data_out = ID_data
					report "ID_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert eq_data_out = (exp_eq_data(I))
					report "eq_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert lt_data_out = (exp_lt_data(I))
					report "lt_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert le_data_out = (exp_le_data(I))
					report "le_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert gt_data_out = (exp_gt_data(I))
					report "gt_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert ge_data_out = (exp_ge_data(I))
					report "ge_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert neq_data_out = (exp_neq_data(I))
					report "neq_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert not_a_data_out = (exp_not_a_data(I))
					report "not_a_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert a_and_b_data_out = (exp_a_and_b_data(I))
					report "a_and_b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert a_or_b_data_out = (exp_a_or_b_data(I))
					report "a_or_b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert a_impl_b_data_out = (exp_a_impl_b_data(I))
					report "a_impl_b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert a_equiv_b_data_out = (exp_a_equiv_b_data(I))
					report "a_equiv_b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert a_xor_b_data_out = (exp_a_xor_b_data(I))
					report "a_xor_b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert true_const_data_out = (exp_true_const_data(I))
					report "true_const_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert time_stream_data_out = std_logic_vector(exp_time_stream_data(I))
					report "time_stream_data differ at iteration I = " & integer'image(I)
					severity Error;
            end if;
        end loop;
        wait;
    end process;

end behaviour;
